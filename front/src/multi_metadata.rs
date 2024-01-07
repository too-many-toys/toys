use std::io::Write;

use web_sys::wasm_bindgen::JsCast;
use web_sys::Blob;
use zip::write::FileOptions;
use zip::ZipWriter;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MultiMetadataWindow {
  pub project_name: String,
  pub is_open: bool,

  metadata: Metadata,
  pub metadata_count: usize,

  pub show_metadata: bool,
}

impl Default for MultiMetadataWindow {
  fn default() -> Self {
    Self {
      project_name: "".to_string(),
      is_open: false,
      metadata: Metadata::default(),
      metadata_count: 0,
      show_metadata: false,
    }
  }
}

impl MultiMetadataWindow {
  pub fn show(&mut self, ui: &mut egui::Ui) {
    ui.checkbox(&mut self.is_open, "단일 메타데이터");
  }

  pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::Window::new("복합 메타데이터 만들기")
      .open(&mut self.is_open)
      .vscroll(true)
      .show(ctx, |ui| {
        ui.label("⚠️현재 버전은 attribute.value 값이 전부 문자열 입니다");
        ui.label("json 파일과 csv 파일을 zip으로 만들어 줍니다");

        ui.horizontal(|ui| {
          ui.label("project name:");
          ui.text_edit_singleline(&mut self.project_name);
        });

        ui.horizontal(|ui| {
          ui.label("metadata count:");
          ui.add(egui::DragValue::new(&mut self.metadata_count));
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

        ui.separator();

        ui.label("Attributes");

        ui.horizontal(|ui| {
          if ui.button("Attribute 추가").clicked() {
            self.metadata.attributes.push(Attribute::default());
          }
          if ui.button("Attribute 제거(맨 밑부터)").clicked() {
            self.metadata.attributes.pop();
          }
        });

        for count in 0..self.metadata.attributes.len() {
          ui.horizontal(|ui| {
            ui.label("trait_type:");
            ui.text_edit_singleline(
              &mut self.metadata.attributes[count].trait_type,
            );
          });
          ui.horizontal(|ui| {
            ui.label("value:");
            ui.text_edit_singleline(&mut self.metadata.attributes[count].value);
          });
        }

        ui.vertical_centered(|ui| {
          if ui.button("Create Metadata").clicked() {
            let mut zip_data: Vec<u8> = Vec::new();
            let mut zip = ZipWriter::new(std::io::Cursor::new(&mut zip_data));

            let mut csv_data: Vec<u8> = Vec::new();
            let mut fields = "name,description,image".to_string();
            for i in 0..self.metadata.attributes.len() {
              fields.push_str(
                format!(",attributes.trait_type_{},attributes.value_{}", i, i)
                  .as_str(),
              );
            }
            fields.push_str("\n");

            csv_data.extend_from_slice(fields.as_bytes());

            for i in 0..self.metadata_count {
              let mut metadata = self.metadata.clone();
              metadata.name = format!("{} #{}", metadata.name, i);

              let metadata_json = serde_json::to_string(&metadata).unwrap();
              let bytes = metadata_json.as_bytes();

              zip
                .start_file(format!("{}.json", i), FileOptions::default())
                .unwrap();
              zip.write_all(&bytes).unwrap();
            }

            let mut csv_line = format!(
              "{},{},{}",
              self.metadata.name,
              self.metadata.description,
              self.metadata.image
            );
            for i in 0..self.metadata.attributes.len() {
              csv_line.push_str(
                format!(
                  ",{},{}",
                  self.metadata.attributes[i].trait_type,
                  self.metadata.attributes[i].value
                )
                .as_str(),
              );
            }
            csv_line.push_str("\n");

            csv_data.extend_from_slice(csv_line.as_bytes());

            zip
              .start_file(
                format!("{}.csv", self.project_name),
                FileOptions::default(),
              )
              .unwrap();
            zip.write_all(&csv_data).unwrap();

            let metadata_files = zip.finish().unwrap().into_inner();

            // #[cfg(target_arch = "wasm32")]
            // {
            let window = web_sys::window().unwrap();
            let doc = window.document().unwrap();

            let uint8arr = web_sys::js_sys::Uint8Array::new(
              &unsafe {
                web_sys::js_sys::Uint8Array::view(metadata_files.by_ref())
              }
              .into(),
            );
            let array = web_sys::js_sys::Array::new();
            array.push(&uint8arr.buffer());

            let blob = Blob::new_with_u8_array_sequence_and_options(
              &array,
              web_sys::BlobPropertyBag::new().type_("application/octet-stream"),
            )
            .unwrap();

            let blob_url =
              web_sys::Url::create_object_url_with_blob(&blob).unwrap();
            let download_link = doc.create_element("a").unwrap();
            let download_link: web_sys::HtmlAnchorElement =
              download_link.unchecked_into();
            download_link.set_href(&blob_url);
            download_link
              .set_download(format!("{}.zip", self.project_name).as_str());
            doc.body().unwrap().append_child(&download_link).unwrap();
            download_link.click();
            // }
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
