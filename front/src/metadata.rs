use egui::CollapsingHeader;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MetadataWindow {
  pub is_open: bool,

  pub metadata: Metadata,
  pub attribute_count: usize,

  pub metadata_mode: MetadataMode,

  pub show_metadata: bool,
}

impl Default for MetadataWindow {
  fn default() -> Self {
    Self {
      is_open: false,
      metadata: Metadata::default(),
      attribute_count: 0,
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
        CollapsingHeader::new("Attributes")
          .default_open(true)
          .show(ui, |ui| {
            let mut i = self.attribute_count;
            ui.add(egui::Slider::new(&mut i, 0..=20).text("count"));

            let mut metadata = vec![Attribute::default(); i];

            for count in 0..i {
              ui.horizontal(|ui| {
                ui.label("trait_type:");
                ui.text_edit_singleline(&mut metadata[count].trait_type);
              });
              ui.horizontal(|ui| {
                ui.label("value:");
                ui.text_edit_singleline(&mut metadata[count].value);
              });
            }
          });

        ui.vertical_centered(|ui| {
          self.show_metadata = ui.button("Create Metadata").clicked();

          if self.show_metadata {
            let mut metadata = self.metadata.clone();
            let mut attributes = metadata.attributes.clone();

            if self.metadata_mode == MetadataMode::Single {
              attributes = vec![Attribute::default(); self.attribute_count];
            }

            metadata.attributes = attributes;

            let mut metadata_json = serde_json::to_string_pretty(&metadata).unwrap();

            egui::Window::new("🔧 Settings")
              .open(&mut self.show_metadata)
              .vscroll(true)
              .show(ctx, |ui| {
                ui.text_edit_multiline(&mut metadata_json);
              });
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

#[derive(serde::Deserialize, serde::Serialize, Clone)]
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
