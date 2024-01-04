use std::collections::BTreeMap;

static MAX_ATTRIBUTE_COUNT: usize = 10;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MetadataWindow {
  pub is_open: bool,

  pub metadata: Metadata,
  pub metadata_count: usize,

  pub attribute_count: usize,

  pub metadata_mode: MetadataMode,

  pub show_metadata: bool,
}

impl Default for MetadataWindow {
  fn default() -> Self {
    Self {
      is_open: false,
      metadata: Metadata::default(),
      metadata_count: 0,
      attribute_count: MAX_ATTRIBUTE_COUNT,
      metadata_mode: MetadataMode::Single,
      show_metadata: false,
    }
  }
}

impl MetadataWindow {
  pub fn show(&mut self, ui: &mut egui::Ui) {
    ui.checkbox(&mut self.is_open, "Create metadata");
  }

  pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::Window::new("메타데이터 만들기")
      .open(&mut self.is_open)
      .vscroll(true)
      .show(ctx, |ui| {
        ui.label("⚠️현재 버전은 attribute.value 값이 전부 문자열 입니다");

        egui::ComboBox::from_label(
          "메타데이터 생성 모드 (Single=같은 메타데이터 생성, Multiple=각자 다른 메타데이터 생성)",
        )
        .selected_text(format!("{:?}", self.metadata_mode))
        .show_ui(ui, |ui| {
          ui.selectable_value(&mut self.metadata_mode, MetadataMode::Single, "Single");
          ui.selectable_value(&mut self.metadata_mode, MetadataMode::Multiple, "Multiple");
        });

        ui.horizontal(|ui| {
          ui.label("name:");
          ui.text_edit_singleline(&mut self.metadata.name);
        });
        ui.horizontal(|ui| {
          ui.label("description:");
          ui.text_edit_singleline(&mut self.metadata.description);
        });
        ui.horizontal(|ui| {
          ui.label("image:");
          ui.text_edit_singleline(&mut self.metadata.image);
        });

        ui.horizontal(|ui| {
          ui.label("attribute count:");
          ui.add(egui::Slider::new(&mut self.attribute_count, 0..=MAX_ATTRIBUTE_COUNT).text("count"));
        });

        ui.label("Attributes");

        self.metadata.attributes.resize(self.attribute_count, Attribute::default());

        for count in 0..self.attribute_count {
          ui.horizontal(|ui| {
            ui.label("trait_type:");
            ui.text_edit_singleline(&mut self.metadata.attributes[count].trait_type);
          });
          ui.horizontal(|ui| {
            ui.label("value:");
            ui.text_edit_singleline(&mut self.metadata.attributes[count].value);
          });
        }

        ui.vertical_centered(|ui| {
          if ui.button("Create Metadata").clicked() {
            let metadata_json = serde_json::to_string(&self.metadata).unwrap();
            log::info!("metadata_json: {}", metadata_json);
            let mut headers = BTreeMap::new();
            headers.insert("Content-Type".to_string(), "application/json".to_string());

            let request = ehttp::Request {
              method: "POST".to_string(),
              url: "http://localhost:3001/json".to_string(),
              headers,
              body: metadata_json.as_bytes().to_vec(),
            };

            ehttp::fetch(request, move |_| {});
          }
        });
      });
  }
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
struct Attribute {
  trait_type: String,
  value: String,
}

impl Default for Attribute {
  fn default() -> Self {
    Self {
      trait_type: "".to_string(),
      value: "".to_string(),
    }
  }
}

#[derive(serde::Deserialize, Clone)]
#[serde(default)]
struct Metadata {
  pub name: String,
  pub description: String,
  pub image: String,
  pub attributes: Vec<Attribute>,
}

impl Default for Metadata {
  fn default() -> Self {
    Self {
      name: "".to_string(),
      description: "".to_string(),
      image: "".to_string(),
      attributes: Vec::new(),
    }
  }
}

impl serde::Serialize for Metadata {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let a = self.attributes.clone();
    let attributes = a
      .into_iter()
      .filter(|v| !v.trait_type.is_empty())
      .collect::<Vec<Attribute>>();
    let mut metadata = self.clone();
    metadata.attributes = attributes;

    metadata.serialize(serializer)
  }
}

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
enum MetadataMode {
  Single,
  Multiple,
}

impl Default for MetadataMode {
  fn default() -> Self {
    Self::Single
  }
}
