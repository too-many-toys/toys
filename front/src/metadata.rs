use std::io::Write;
use std::ops::{Deref, DerefMut};

use web_sys::wasm_bindgen::JsCast;
use web_sys::{Blob, File};
use zip::write::FileOptions;
use zip::ZipWriter;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct SingleMetadataWindow {
  pub project_name: String,
  pub is_open: bool,

  metadata: Metadata,
  pub metadata_count: usize,

  pub show_metadata: bool,
}

impl Default for SingleMetadataWindow {
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

impl SingleMetadataWindow {
  pub fn show(&mut self, ui: &mut egui::Ui) {
    ui.checkbox(&mut self.is_open, "단일 메타데이터");
  }

  pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::Window::new("단일 메타데이터 만들기")
      .open(&mut self.is_open)
      .vscroll(true)
      .show(ctx, |ui| {
        ui.label("⚠️현재 버전은 attribute.value 값이 전부 문자열 입니다");
        ui.label("json 파일과 xlsx 파일을 zip으로 만들어 줍니다");

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

// impl serde::Serialize for Metadata {
//   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//   where
//     S: serde::Serializer,
//   {
//     let a = self.attributes.clone();
//     let attributes = a
//       .into_iter()
//       .filter(|v| !v.trait_type.is_empty())
//       .collect::<Vec<Attribute>>();
//     let mut metadata = self.clone();
//     metadata.attributes = attributes;

//     metadata.serialize(serializer)
//   }
// }

// let mut headers = BTreeMap::new();
//   headers.insert(
//     "Content-Type".to_string(),
//     "application/json".to_string(),
// );

// let request = ehttp::Request {
//   method: "POST".to_string(),
//   url: "http://localhost:3001/json".to_string(),
//   headers,
//   body: metadata_json.as_bytes().to_vec(),
// };

// let bytes = metadata_json.as_bytes();

// let uint8arr = web_sys::js_sys::Uint8Array::new(
//   &unsafe { web_sys::js_sys::Uint8Array::view(&bytes) }.into(),
// );
// let array = web_sys::js_sys::Array::new();
// array.push(&uint8arr.buffer());

// let blob = Blob::new_with_str_sequence(&array).unwrap();

// let blob_url = Url::create_object_url_with_blob(&blob).unwrap();
// ui.ctx().open_url(egui::OpenUrl {
//   url: blob_url,
//   new_tab: true,
// });
// ehttp::fetch(request, move |_| {});
