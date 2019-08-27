pub mod schemas {
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct AnnotateFileResponse {
        #[doc = "Information about the file for which this response is generated."]
        #[serde(rename = "inputConfig", default)]
        pub input_config: ::std::option::Option<crate::schemas::InputConfig>,
        #[doc = "Individual responses to images found within the file. This field will be\nempty if the `error` field is set."]
        #[serde(rename = "responses", default)]
        pub responses: ::std::option::Option<Vec<crate::schemas::AnnotateImageResponse>>,
        #[doc = "This field gives the total number of pages in the file."]
        #[serde(rename = "totalPages", default)]
        pub total_pages: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for AnnotateFileResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct AnnotateImageResponse {
        #[doc = "If present, contextual information is needed to understand where this image\ncomes from."]
        #[serde(rename = "context", default)]
        pub context: ::std::option::Option<crate::schemas::ImageAnnotationContext>,
        #[doc = "If present, crop hints have completed successfully."]
        #[serde(rename = "cropHintsAnnotation", default)]
        pub crop_hints_annotation: ::std::option::Option<crate::schemas::CropHintsAnnotation>,
        #[doc = "If set, represents the error message for the operation.\nNote that filled-in image annotations are guaranteed to be\ncorrect, even when `error` is set."]
        #[serde(rename = "error", default)]
        pub error: ::std::option::Option<crate::schemas::Status>,
        #[doc = "If present, face detection has completed successfully."]
        #[serde(rename = "faceAnnotations", default)]
        pub face_annotations: ::std::option::Option<Vec<crate::schemas::FaceAnnotation>>,
        #[doc = "If present, text (OCR) detection or document (OCR) text detection has\ncompleted successfully.\nThis annotation provides the structural hierarchy for the OCR detected\ntext."]
        #[serde(rename = "fullTextAnnotation", default)]
        pub full_text_annotation: ::std::option::Option<crate::schemas::TextAnnotation>,
        #[doc = "If present, image properties were extracted successfully."]
        #[serde(rename = "imagePropertiesAnnotation", default)]
        pub image_properties_annotation: ::std::option::Option<crate::schemas::ImageProperties>,
        #[doc = "If present, label detection has completed successfully."]
        #[serde(rename = "labelAnnotations", default)]
        pub label_annotations: ::std::option::Option<Vec<crate::schemas::EntityAnnotation>>,
        #[doc = "If present, landmark detection has completed successfully."]
        #[serde(rename = "landmarkAnnotations", default)]
        pub landmark_annotations: ::std::option::Option<Vec<crate::schemas::EntityAnnotation>>,
        #[doc = "If present, localized object detection has completed successfully.\nThis will be sorted descending by confidence score."]
        #[serde(rename = "localizedObjectAnnotations", default)]
        pub localized_object_annotations:
            ::std::option::Option<Vec<crate::schemas::LocalizedObjectAnnotation>>,
        #[doc = "If present, logo detection has completed successfully."]
        #[serde(rename = "logoAnnotations", default)]
        pub logo_annotations: ::std::option::Option<Vec<crate::schemas::EntityAnnotation>>,
        #[doc = "If present, product search has completed successfully."]
        #[serde(rename = "productSearchResults", default)]
        pub product_search_results: ::std::option::Option<crate::schemas::ProductSearchResults>,
        #[doc = "If present, safe-search annotation has completed successfully."]
        #[serde(rename = "safeSearchAnnotation", default)]
        pub safe_search_annotation: ::std::option::Option<crate::schemas::SafeSearchAnnotation>,
        #[doc = "If present, text (OCR) detection has completed successfully."]
        #[serde(rename = "textAnnotations", default)]
        pub text_annotations: ::std::option::Option<Vec<crate::schemas::EntityAnnotation>>,
        #[doc = "If present, web detection has completed successfully."]
        #[serde(rename = "webDetection", default)]
        pub web_detection: ::std::option::Option<crate::schemas::WebDetection>,
    }
    impl ::field_selector::FieldSelector for AnnotateImageResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AsyncAnnotateFileResponse {
        #[doc = "The output location and metadata from AsyncAnnotateFileRequest."]
        #[serde(rename = "outputConfig", default)]
        pub output_config: ::std::option::Option<crate::schemas::OutputConfig>,
    }
    impl ::field_selector::FieldSelector for AsyncAnnotateFileResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AsyncBatchAnnotateFilesResponse {
        #[doc = "The list of file annotation responses, one for each request in\nAsyncBatchAnnotateFilesRequest."]
        #[serde(rename = "responses", default)]
        pub responses: ::std::option::Option<Vec<crate::schemas::AsyncAnnotateFileResponse>>,
    }
    impl ::field_selector::FieldSelector for AsyncBatchAnnotateFilesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AsyncBatchAnnotateImagesResponse {
        #[doc = "The output location and metadata from AsyncBatchAnnotateImagesRequest."]
        #[serde(rename = "outputConfig", default)]
        pub output_config: ::std::option::Option<crate::schemas::OutputConfig>,
    }
    impl ::field_selector::FieldSelector for AsyncBatchAnnotateImagesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct BatchAnnotateFilesResponse {
        #[doc = "The list of file annotation responses, each response corresponding to each\nAnnotateFileRequest in BatchAnnotateFilesRequest."]
        #[serde(rename = "responses", default)]
        pub responses: ::std::option::Option<Vec<crate::schemas::AnnotateFileResponse>>,
    }
    impl ::field_selector::FieldSelector for BatchAnnotateFilesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct BatchOperationMetadata {
        #[doc = "The time when the batch request is finished and\ngoogle.longrunning.Operation.done is set to true."]
        #[serde(rename = "endTime", default)]
        pub end_time: ::std::option::Option<String>,
        #[doc = "The current state of the batch operation."]
        #[serde(rename = "state", default)]
        pub state: ::std::option::Option<crate::schemas::BatchOperationMetadataState>,
        #[doc = "The time when the batch request was submitted to the server."]
        #[serde(rename = "submitTime", default)]
        pub submit_time: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for BatchOperationMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BatchOperationMetadataState {
        #[doc = "The request is done after the longrunning.Operations.CancelOperation has\nbeen called by the user.  Any records that were processed before the\ncancel command are output as specified in the request."]
        Cancelled,
        #[doc = "The request is done and no item has been successfully processed."]
        Failed,
        #[doc = "Request is actively being processed."]
        Processing,
        #[doc = "Invalid."]
        StateUnspecified,
        #[doc = "The request is done and at least one item has been successfully\nprocessed."]
        Successful,
    }
    impl BatchOperationMetadataState {
        pub fn as_str(self) -> &'static str {
            match self {
                BatchOperationMetadataState::Cancelled => "CANCELLED",
                BatchOperationMetadataState::Failed => "FAILED",
                BatchOperationMetadataState::Processing => "PROCESSING",
                BatchOperationMetadataState::StateUnspecified => "STATE_UNSPECIFIED",
                BatchOperationMetadataState::Successful => "SUCCESSFUL",
            }
        }
    }
    impl ::std::fmt::Display for BatchOperationMetadataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BatchOperationMetadataState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BatchOperationMetadataState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => BatchOperationMetadataState::Cancelled,
                "FAILED" => BatchOperationMetadataState::Failed,
                "PROCESSING" => BatchOperationMetadataState::Processing,
                "STATE_UNSPECIFIED" => BatchOperationMetadataState::StateUnspecified,
                "SUCCESSFUL" => BatchOperationMetadataState::Successful,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for BatchOperationMetadataState {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Block {
        #[doc = "Detected block type (text, image etc) for this block."]
        #[serde(rename = "blockType", default)]
        pub block_type: ::std::option::Option<crate::schemas::BlockBlockType>,
        #[doc = "The bounding box for the block.\nThe vertices are in the order of top-left, top-right, bottom-right,\nbottom-left. When a rotation of the bounding box is detected the rotation\nis represented as around the top-left corner as defined when the text is\nread in the 'natural' orientation.\nFor example:\n\n* when the text is horizontal it might look like:\n  \n  ````text\n    0----1\n    |    |\n    3----2\n  ````\n\n* when it's rotated 180 degrees around the top-left corner it becomes:\n  \n  ````text\n    2----3\n    |    |\n    1----0\n  ````\n  \n  and the vertex order will still be (0, 1, 2, 3)."]
        #[serde(rename = "boundingBox", default)]
        pub bounding_box: ::std::option::Option<crate::schemas::BoundingPoly>,
        #[doc = "Confidence of the OCR results on the block. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "List of paragraphs in this block (if this blocks is of type text)."]
        #[serde(rename = "paragraphs", default)]
        pub paragraphs: ::std::option::Option<Vec<crate::schemas::Paragraph>>,
        #[doc = "Additional information detected for the block."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<crate::schemas::TextProperty>,
    }
    impl ::field_selector::FieldSelector for Block {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BlockBlockType {
        #[doc = "Barcode block."]
        Barcode,
        #[doc = "Image block."]
        Picture,
        #[doc = "Horizontal/vertical line box."]
        Ruler,
        #[doc = "Table block."]
        Table,
        #[doc = "Regular text block."]
        Text,
        #[doc = "Unknown block type."]
        Unknown,
    }
    impl BlockBlockType {
        pub fn as_str(self) -> &'static str {
            match self {
                BlockBlockType::Barcode => "BARCODE",
                BlockBlockType::Picture => "PICTURE",
                BlockBlockType::Ruler => "RULER",
                BlockBlockType::Table => "TABLE",
                BlockBlockType::Text => "TEXT",
                BlockBlockType::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for BlockBlockType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BlockBlockType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BlockBlockType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BARCODE" => BlockBlockType::Barcode,
                "PICTURE" => BlockBlockType::Picture,
                "RULER" => BlockBlockType::Ruler,
                "TABLE" => BlockBlockType::Table,
                "TEXT" => BlockBlockType::Text,
                "UNKNOWN" => BlockBlockType::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for BlockBlockType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BoundingPoly {
        #[doc = "The bounding polygon normalized vertices."]
        #[serde(rename = "normalizedVertices", default)]
        pub normalized_vertices: ::std::option::Option<Vec<crate::schemas::NormalizedVertex>>,
        #[doc = "The bounding polygon vertices."]
        #[serde(rename = "vertices", default)]
        pub vertices: ::std::option::Option<Vec<crate::schemas::Vertex>>,
    }
    impl ::field_selector::FieldSelector for BoundingPoly {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Color {
        #[doc = "The fraction of this color that should be applied to the pixel. That is,\nthe final pixel color is defined by the equation:\n\npixel color = alpha * (this color) + (1.0 - alpha) * (background color)\n\nThis means that a value of 1.0 corresponds to a solid color, whereas\na value of 0.0 corresponds to a completely transparent color. This\nuses a wrapper message rather than a simple float scalar so that it is\npossible to distinguish between a default value and the value being unset.\nIf omitted, this color object is to be rendered as a solid color\n(as if the alpha value had been explicitly given with a value of 1.0)."]
        #[serde(rename = "alpha", default)]
        pub alpha: ::std::option::Option<f32>,
        #[doc = "The amount of blue in the color as a value in the interval [0, 1]."]
        #[serde(rename = "blue", default)]
        pub blue: ::std::option::Option<f32>,
        #[doc = "The amount of green in the color as a value in the interval [0, 1]."]
        #[serde(rename = "green", default)]
        pub green: ::std::option::Option<f32>,
        #[doc = "The amount of red in the color as a value in the interval [0, 1]."]
        #[serde(rename = "red", default)]
        pub red: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for Color {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ColorInfo {
        #[doc = "RGB components of the color."]
        #[serde(rename = "color", default)]
        pub color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The fraction of pixels the color occupies in the image.\nValue in range [0, 1]."]
        #[serde(rename = "pixelFraction", default)]
        pub pixel_fraction: ::std::option::Option<f32>,
        #[doc = "Image-specific score for this color. Value in range [0, 1]."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for ColorInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CropHint {
        #[doc = "The bounding polygon for the crop region. The coordinates of the bounding\nbox are in the original image's scale."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly: ::std::option::Option<crate::schemas::BoundingPoly>,
        #[doc = "Confidence of this being a salient region.  Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Fraction of importance of this salient region with respect to the original\nimage."]
        #[serde(rename = "importanceFraction", default)]
        pub importance_fraction: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for CropHint {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CropHintsAnnotation {
        #[doc = "Crop hint results."]
        #[serde(rename = "cropHints", default)]
        pub crop_hints: ::std::option::Option<Vec<crate::schemas::CropHint>>,
    }
    impl ::field_selector::FieldSelector for CropHintsAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DetectedBreak {
        #[doc = "True if break prepends the element."]
        #[serde(rename = "isPrefix", default)]
        pub is_prefix: ::std::option::Option<bool>,
        #[doc = "Detected break type."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::DetectedBreakType>,
    }
    impl ::field_selector::FieldSelector for DetectedBreak {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DetectedBreakType {
        #[doc = "Line-wrapping break."]
        EolSureSpace,
        #[doc = "End-line hyphen that is not present in text; does not co-occur with\n`SPACE`, `LEADER_SPACE`, or `LINE_BREAK`."]
        Hyphen,
        #[doc = "Line break that ends a paragraph."]
        LineBreak,
        #[doc = "Regular space."]
        Space,
        #[doc = "Sure space (very wide)."]
        SureSpace,
        #[doc = "Unknown break label type."]
        Unknown,
    }
    impl DetectedBreakType {
        pub fn as_str(self) -> &'static str {
            match self {
                DetectedBreakType::EolSureSpace => "EOL_SURE_SPACE",
                DetectedBreakType::Hyphen => "HYPHEN",
                DetectedBreakType::LineBreak => "LINE_BREAK",
                DetectedBreakType::Space => "SPACE",
                DetectedBreakType::SureSpace => "SURE_SPACE",
                DetectedBreakType::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for DetectedBreakType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DetectedBreakType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DetectedBreakType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EOL_SURE_SPACE" => DetectedBreakType::EolSureSpace,
                "HYPHEN" => DetectedBreakType::Hyphen,
                "LINE_BREAK" => DetectedBreakType::LineBreak,
                "SPACE" => DetectedBreakType::Space,
                "SURE_SPACE" => DetectedBreakType::SureSpace,
                "UNKNOWN" => DetectedBreakType::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for DetectedBreakType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DetectedLanguage {
        #[doc = "Confidence of detected language. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more\ninformation, see\nhttp://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for DetectedLanguage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DominantColorsAnnotation {
        #[doc = "RGB color values with their score and pixel fraction."]
        #[serde(rename = "colors", default)]
        pub colors: ::std::option::Option<Vec<crate::schemas::ColorInfo>>,
    }
    impl ::field_selector::FieldSelector for DominantColorsAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct EntityAnnotation {
        #[doc = "Image region to which this entity belongs. Not produced\nfor `LABEL_DETECTION` features."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly: ::std::option::Option<crate::schemas::BoundingPoly>,
        #[doc = "**Deprecated. Use `score` instead.**\nThe accuracy of the entity detection in an image.\nFor example, for an image in which the \"Eiffel Tower\" entity is detected,\nthis field represents the confidence that there is a tower in the query\nimage. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Entity textual description, expressed in its `locale` language."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "The language code for the locale in which the entity textual\n`description` is expressed."]
        #[serde(rename = "locale", default)]
        pub locale: ::std::option::Option<String>,
        #[doc = "The location information for the detected entity. Multiple\n`LocationInfo` elements can be present because one location may\nindicate the location of the scene in the image, and another location\nmay indicate the location of the place where the image was taken.\nLocation information is usually present for landmarks."]
        #[serde(rename = "locations", default)]
        pub locations: ::std::option::Option<Vec<crate::schemas::LocationInfo>>,
        #[doc = "Opaque entity ID. Some IDs may be available in\n[Google Knowledge Graph Search\nAPI](https://developers.google.com/knowledge-graph/)."]
        #[serde(rename = "mid", default)]
        pub mid: ::std::option::Option<String>,
        #[doc = "Some entities may have optional user-supplied `Property` (name/value)\nfields, such a score or string that qualifies the entity."]
        #[serde(rename = "properties", default)]
        pub properties: ::std::option::Option<Vec<crate::schemas::Property>>,
        #[doc = "Overall score of the result. Range [0, 1]."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
        #[doc = "The relevancy of the ICA (Image Content Annotation) label to the\nimage. For example, the relevancy of \"tower\" is likely higher to an image\ncontaining the detected \"Eiffel Tower\" than to an image containing a\ndetected distant towering building, even though the confidence that\nthere is a tower in each image may be the same. Range [0, 1]."]
        #[serde(rename = "topicality", default)]
        pub topicality: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for EntityAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct FaceAnnotation {
        #[doc = "Anger likelihood."]
        #[serde(rename = "angerLikelihood", default)]
        pub anger_likelihood: ::std::option::Option<crate::schemas::FaceAnnotationAngerLikelihood>,
        #[doc = "Blurred likelihood."]
        #[serde(rename = "blurredLikelihood", default)]
        pub blurred_likelihood:
            ::std::option::Option<crate::schemas::FaceAnnotationBlurredLikelihood>,
        #[doc = "The bounding polygon around the face. The coordinates of the bounding box\nare in the original image's scale.\nThe bounding box is computed to \"frame\" the face in accordance with human\nexpectations. It is based on the landmarker results.\nNote that one or more x and/or y coordinates may not be generated in the\n`BoundingPoly` (the polygon will be unbounded) if only a partial face\nappears in the image to be annotated."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly: ::std::option::Option<crate::schemas::BoundingPoly>,
        #[doc = "Detection confidence. Range [0, 1]."]
        #[serde(rename = "detectionConfidence", default)]
        pub detection_confidence: ::std::option::Option<f32>,
        #[doc = "The `fd_bounding_poly` bounding polygon is tighter than the\n`boundingPoly`, and encloses only the skin part of the face. Typically, it\nis used to eliminate the face from any image analysis that detects the\n\"amount of skin\" visible in an image. It is not based on the\nlandmarker results, only on the initial face detection, hence\nthe <code>fd</code> (face detection) prefix."]
        #[serde(rename = "fdBoundingPoly", default)]
        pub fd_bounding_poly: ::std::option::Option<crate::schemas::BoundingPoly>,
        #[doc = "Headwear likelihood."]
        #[serde(rename = "headwearLikelihood", default)]
        pub headwear_likelihood:
            ::std::option::Option<crate::schemas::FaceAnnotationHeadwearLikelihood>,
        #[doc = "Joy likelihood."]
        #[serde(rename = "joyLikelihood", default)]
        pub joy_likelihood: ::std::option::Option<crate::schemas::FaceAnnotationJoyLikelihood>,
        #[doc = "Face landmarking confidence. Range [0, 1]."]
        #[serde(rename = "landmarkingConfidence", default)]
        pub landmarking_confidence: ::std::option::Option<f32>,
        #[doc = "Detected face landmarks."]
        #[serde(rename = "landmarks", default)]
        pub landmarks: ::std::option::Option<Vec<crate::schemas::Landmark>>,
        #[doc = "Yaw angle, which indicates the leftward/rightward angle that the face is\npointing relative to the vertical plane perpendicular to the image. Range\n[-180,180]."]
        #[serde(rename = "panAngle", default)]
        pub pan_angle: ::std::option::Option<f32>,
        #[doc = "Roll angle, which indicates the amount of clockwise/anti-clockwise rotation\nof the face relative to the image vertical about the axis perpendicular to\nthe face. Range [-180,180]."]
        #[serde(rename = "rollAngle", default)]
        pub roll_angle: ::std::option::Option<f32>,
        #[doc = "Sorrow likelihood."]
        #[serde(rename = "sorrowLikelihood", default)]
        pub sorrow_likelihood:
            ::std::option::Option<crate::schemas::FaceAnnotationSorrowLikelihood>,
        #[doc = "Surprise likelihood."]
        #[serde(rename = "surpriseLikelihood", default)]
        pub surprise_likelihood:
            ::std::option::Option<crate::schemas::FaceAnnotationSurpriseLikelihood>,
        #[doc = "Pitch angle, which indicates the upwards/downwards angle that the face is\npointing relative to the image's horizontal plane. Range [-180,180]."]
        #[serde(rename = "tiltAngle", default)]
        pub tilt_angle: ::std::option::Option<f32>,
        #[doc = "Under-exposed likelihood."]
        #[serde(rename = "underExposedLikelihood", default)]
        pub under_exposed_likelihood:
            ::std::option::Option<crate::schemas::FaceAnnotationUnderExposedLikelihood>,
    }
    impl ::field_selector::FieldSelector for FaceAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FaceAnnotationAngerLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl FaceAnnotationAngerLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                FaceAnnotationAngerLikelihood::Likely => "LIKELY",
                FaceAnnotationAngerLikelihood::Possible => "POSSIBLE",
                FaceAnnotationAngerLikelihood::Unknown => "UNKNOWN",
                FaceAnnotationAngerLikelihood::Unlikely => "UNLIKELY",
                FaceAnnotationAngerLikelihood::VeryLikely => "VERY_LIKELY",
                FaceAnnotationAngerLikelihood::VeryUnlikely => "VERY_UNLIKELY",
            }
        }
    }
    impl ::std::fmt::Display for FaceAnnotationAngerLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FaceAnnotationAngerLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FaceAnnotationAngerLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => FaceAnnotationAngerLikelihood::Likely,
                "POSSIBLE" => FaceAnnotationAngerLikelihood::Possible,
                "UNKNOWN" => FaceAnnotationAngerLikelihood::Unknown,
                "UNLIKELY" => FaceAnnotationAngerLikelihood::Unlikely,
                "VERY_LIKELY" => FaceAnnotationAngerLikelihood::VeryLikely,
                "VERY_UNLIKELY" => FaceAnnotationAngerLikelihood::VeryUnlikely,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for FaceAnnotationAngerLikelihood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FaceAnnotationBlurredLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl FaceAnnotationBlurredLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                FaceAnnotationBlurredLikelihood::Likely => "LIKELY",
                FaceAnnotationBlurredLikelihood::Possible => "POSSIBLE",
                FaceAnnotationBlurredLikelihood::Unknown => "UNKNOWN",
                FaceAnnotationBlurredLikelihood::Unlikely => "UNLIKELY",
                FaceAnnotationBlurredLikelihood::VeryLikely => "VERY_LIKELY",
                FaceAnnotationBlurredLikelihood::VeryUnlikely => "VERY_UNLIKELY",
            }
        }
    }
    impl ::std::fmt::Display for FaceAnnotationBlurredLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FaceAnnotationBlurredLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FaceAnnotationBlurredLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => FaceAnnotationBlurredLikelihood::Likely,
                "POSSIBLE" => FaceAnnotationBlurredLikelihood::Possible,
                "UNKNOWN" => FaceAnnotationBlurredLikelihood::Unknown,
                "UNLIKELY" => FaceAnnotationBlurredLikelihood::Unlikely,
                "VERY_LIKELY" => FaceAnnotationBlurredLikelihood::VeryLikely,
                "VERY_UNLIKELY" => FaceAnnotationBlurredLikelihood::VeryUnlikely,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for FaceAnnotationBlurredLikelihood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FaceAnnotationHeadwearLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl FaceAnnotationHeadwearLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                FaceAnnotationHeadwearLikelihood::Likely => "LIKELY",
                FaceAnnotationHeadwearLikelihood::Possible => "POSSIBLE",
                FaceAnnotationHeadwearLikelihood::Unknown => "UNKNOWN",
                FaceAnnotationHeadwearLikelihood::Unlikely => "UNLIKELY",
                FaceAnnotationHeadwearLikelihood::VeryLikely => "VERY_LIKELY",
                FaceAnnotationHeadwearLikelihood::VeryUnlikely => "VERY_UNLIKELY",
            }
        }
    }
    impl ::std::fmt::Display for FaceAnnotationHeadwearLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FaceAnnotationHeadwearLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FaceAnnotationHeadwearLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => FaceAnnotationHeadwearLikelihood::Likely,
                "POSSIBLE" => FaceAnnotationHeadwearLikelihood::Possible,
                "UNKNOWN" => FaceAnnotationHeadwearLikelihood::Unknown,
                "UNLIKELY" => FaceAnnotationHeadwearLikelihood::Unlikely,
                "VERY_LIKELY" => FaceAnnotationHeadwearLikelihood::VeryLikely,
                "VERY_UNLIKELY" => FaceAnnotationHeadwearLikelihood::VeryUnlikely,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for FaceAnnotationHeadwearLikelihood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FaceAnnotationJoyLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl FaceAnnotationJoyLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                FaceAnnotationJoyLikelihood::Likely => "LIKELY",
                FaceAnnotationJoyLikelihood::Possible => "POSSIBLE",
                FaceAnnotationJoyLikelihood::Unknown => "UNKNOWN",
                FaceAnnotationJoyLikelihood::Unlikely => "UNLIKELY",
                FaceAnnotationJoyLikelihood::VeryLikely => "VERY_LIKELY",
                FaceAnnotationJoyLikelihood::VeryUnlikely => "VERY_UNLIKELY",
            }
        }
    }
    impl ::std::fmt::Display for FaceAnnotationJoyLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FaceAnnotationJoyLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FaceAnnotationJoyLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => FaceAnnotationJoyLikelihood::Likely,
                "POSSIBLE" => FaceAnnotationJoyLikelihood::Possible,
                "UNKNOWN" => FaceAnnotationJoyLikelihood::Unknown,
                "UNLIKELY" => FaceAnnotationJoyLikelihood::Unlikely,
                "VERY_LIKELY" => FaceAnnotationJoyLikelihood::VeryLikely,
                "VERY_UNLIKELY" => FaceAnnotationJoyLikelihood::VeryUnlikely,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for FaceAnnotationJoyLikelihood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FaceAnnotationSorrowLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl FaceAnnotationSorrowLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                FaceAnnotationSorrowLikelihood::Likely => "LIKELY",
                FaceAnnotationSorrowLikelihood::Possible => "POSSIBLE",
                FaceAnnotationSorrowLikelihood::Unknown => "UNKNOWN",
                FaceAnnotationSorrowLikelihood::Unlikely => "UNLIKELY",
                FaceAnnotationSorrowLikelihood::VeryLikely => "VERY_LIKELY",
                FaceAnnotationSorrowLikelihood::VeryUnlikely => "VERY_UNLIKELY",
            }
        }
    }
    impl ::std::fmt::Display for FaceAnnotationSorrowLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FaceAnnotationSorrowLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FaceAnnotationSorrowLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => FaceAnnotationSorrowLikelihood::Likely,
                "POSSIBLE" => FaceAnnotationSorrowLikelihood::Possible,
                "UNKNOWN" => FaceAnnotationSorrowLikelihood::Unknown,
                "UNLIKELY" => FaceAnnotationSorrowLikelihood::Unlikely,
                "VERY_LIKELY" => FaceAnnotationSorrowLikelihood::VeryLikely,
                "VERY_UNLIKELY" => FaceAnnotationSorrowLikelihood::VeryUnlikely,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for FaceAnnotationSorrowLikelihood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FaceAnnotationSurpriseLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl FaceAnnotationSurpriseLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                FaceAnnotationSurpriseLikelihood::Likely => "LIKELY",
                FaceAnnotationSurpriseLikelihood::Possible => "POSSIBLE",
                FaceAnnotationSurpriseLikelihood::Unknown => "UNKNOWN",
                FaceAnnotationSurpriseLikelihood::Unlikely => "UNLIKELY",
                FaceAnnotationSurpriseLikelihood::VeryLikely => "VERY_LIKELY",
                FaceAnnotationSurpriseLikelihood::VeryUnlikely => "VERY_UNLIKELY",
            }
        }
    }
    impl ::std::fmt::Display for FaceAnnotationSurpriseLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FaceAnnotationSurpriseLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FaceAnnotationSurpriseLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => FaceAnnotationSurpriseLikelihood::Likely,
                "POSSIBLE" => FaceAnnotationSurpriseLikelihood::Possible,
                "UNKNOWN" => FaceAnnotationSurpriseLikelihood::Unknown,
                "UNLIKELY" => FaceAnnotationSurpriseLikelihood::Unlikely,
                "VERY_LIKELY" => FaceAnnotationSurpriseLikelihood::VeryLikely,
                "VERY_UNLIKELY" => FaceAnnotationSurpriseLikelihood::VeryUnlikely,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for FaceAnnotationSurpriseLikelihood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FaceAnnotationUnderExposedLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl FaceAnnotationUnderExposedLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                FaceAnnotationUnderExposedLikelihood::Likely => "LIKELY",
                FaceAnnotationUnderExposedLikelihood::Possible => "POSSIBLE",
                FaceAnnotationUnderExposedLikelihood::Unknown => "UNKNOWN",
                FaceAnnotationUnderExposedLikelihood::Unlikely => "UNLIKELY",
                FaceAnnotationUnderExposedLikelihood::VeryLikely => "VERY_LIKELY",
                FaceAnnotationUnderExposedLikelihood::VeryUnlikely => "VERY_UNLIKELY",
            }
        }
    }
    impl ::std::fmt::Display for FaceAnnotationUnderExposedLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FaceAnnotationUnderExposedLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FaceAnnotationUnderExposedLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => FaceAnnotationUnderExposedLikelihood::Likely,
                "POSSIBLE" => FaceAnnotationUnderExposedLikelihood::Possible,
                "UNKNOWN" => FaceAnnotationUnderExposedLikelihood::Unknown,
                "UNLIKELY" => FaceAnnotationUnderExposedLikelihood::Unlikely,
                "VERY_LIKELY" => FaceAnnotationUnderExposedLikelihood::VeryLikely,
                "VERY_UNLIKELY" => FaceAnnotationUnderExposedLikelihood::VeryUnlikely,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for FaceAnnotationUnderExposedLikelihood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GcsDestination {
        #[doc = "Google Cloud Storage URI prefix where the results will be stored. Results\nwill be in JSON format and preceded by its corresponding input URI prefix.\nThis field can either represent a gcs file prefix or gcs directory. In\neither case, the uri should be unique because in order to get all of the\noutput files, you will need to do a wildcard gcs search on the uri prefix\nyou provide.\n\nExamples:\n\n* File Prefix: gs://bucket-name/here/filenameprefix   The output files\n  will be created in gs://bucket-name/here/ and the names of the\n  output files will begin with \"filenameprefix\".\n\n* Directory Prefix: gs://bucket-name/some/location/   The output files\n  will be created in gs://bucket-name/some/location/ and the names of the\n  output files could be anything because there was no filename prefix\n  specified.\n\nIf multiple outputs, each response is still AnnotateFileResponse, each of\nwhich contains some subset of the full list of AnnotateImageResponse.\nMultiple outputs can happen if, for example, the output JSON is too large\nand overflows into multiple sharded files."]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GcsDestination {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GcsSource {
        #[doc = "Google Cloud Storage URI for the input file. This must only be a\nGoogle Cloud Storage object. Wildcards are not currently supported."]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GcsSource {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1AnnotateFileRequest {
        #[doc = "Required. Requested features."]
        #[serde(rename = "features", default)]
        pub features: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P1Beta1Feature>>,
        #[doc = "Additional context that may accompany the image(s) in the file."]
        #[serde(rename = "imageContext", default)]
        pub image_context:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1ImageContext>,
        #[doc = "Required. Information about the input file."]
        #[serde(rename = "inputConfig", default)]
        pub input_config:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1InputConfig>,
        #[doc = "Pages of the file to perform image annotation.\n\nPages starts from 1, we assume the first page of the file is page 1.\nAt most 5 pages are supported per request. Pages can be negative.\n\nPage 1 means the first page.\nPage 2 means the second page.\nPage -1 means the last page.\nPage -2 means the second to the last page.\n\nIf the file is GIF instead of PDF or TIFF, page refers to GIF frames.\n\nIf this field is empty, by default the service performs image annotation\nfor the first 5 pages of the file."]
        #[serde(rename = "pages", default)]
        pub pages: ::std::option::Option<Vec<i32>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1AnnotateFileRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVisionV1P1Beta1AnnotateFileResponse {
        #[doc = "Information about the file for which this response is generated."]
        #[serde(rename = "inputConfig", default)]
        pub input_config:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1InputConfig>,
        #[doc = "Individual responses to images found within the file. This field will be\nempty if the `error` field is set."]
        #[serde(rename = "responses", default)]
        pub responses: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P1Beta1AnnotateImageResponse>,
        >,
        #[doc = "This field gives the total number of pages in the file."]
        #[serde(rename = "totalPages", default)]
        pub total_pages: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1AnnotateFileResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1AnnotateImageRequest {
        #[doc = "Requested features."]
        #[serde(rename = "features", default)]
        pub features: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P1Beta1Feature>>,
        #[doc = "The image to be processed."]
        #[serde(rename = "image", default)]
        pub image: ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1Image>,
        #[doc = "Additional context that may accompany the image."]
        #[serde(rename = "imageContext", default)]
        pub image_context:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1ImageContext>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1AnnotateImageRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVisionV1P1Beta1AnnotateImageResponse {
        #[doc = "If present, contextual information is needed to understand where this image\ncomes from."]
        #[serde(rename = "context", default)]
        pub context:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1ImageAnnotationContext>,
        #[doc = "If present, crop hints have completed successfully."]
        #[serde(rename = "cropHintsAnnotation", default)]
        pub crop_hints_annotation:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1CropHintsAnnotation>,
        #[doc = "If set, represents the error message for the operation.\nNote that filled-in image annotations are guaranteed to be\ncorrect, even when `error` is set."]
        #[serde(rename = "error", default)]
        pub error: ::std::option::Option<crate::schemas::Status>,
        #[doc = "If present, face detection has completed successfully."]
        #[serde(rename = "faceAnnotations", default)]
        pub face_annotations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P1Beta1FaceAnnotation>>,
        #[doc = "If present, text (OCR) detection or document (OCR) text detection has\ncompleted successfully.\nThis annotation provides the structural hierarchy for the OCR detected\ntext."]
        #[serde(rename = "fullTextAnnotation", default)]
        pub full_text_annotation:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1TextAnnotation>,
        #[doc = "If present, image properties were extracted successfully."]
        #[serde(rename = "imagePropertiesAnnotation", default)]
        pub image_properties_annotation:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1ImageProperties>,
        #[doc = "If present, label detection has completed successfully."]
        #[serde(rename = "labelAnnotations", default)]
        pub label_annotations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P1Beta1EntityAnnotation>>,
        #[doc = "If present, landmark detection has completed successfully."]
        #[serde(rename = "landmarkAnnotations", default)]
        pub landmark_annotations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P1Beta1EntityAnnotation>>,
        #[doc = "If present, localized object detection has completed successfully.\nThis will be sorted descending by confidence score."]
        #[serde(rename = "localizedObjectAnnotations", default)]
        pub localized_object_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P1Beta1LocalizedObjectAnnotation>,
        >,
        #[doc = "If present, logo detection has completed successfully."]
        #[serde(rename = "logoAnnotations", default)]
        pub logo_annotations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P1Beta1EntityAnnotation>>,
        #[doc = "If present, product search has completed successfully."]
        #[serde(rename = "productSearchResults", default)]
        pub product_search_results:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1ProductSearchResults>,
        #[doc = "If present, safe-search annotation has completed successfully."]
        #[serde(rename = "safeSearchAnnotation", default)]
        pub safe_search_annotation:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1SafeSearchAnnotation>,
        #[doc = "If present, text (OCR) detection has completed successfully."]
        #[serde(rename = "textAnnotations", default)]
        pub text_annotations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P1Beta1EntityAnnotation>>,
        #[doc = "If present, web detection has completed successfully."]
        #[serde(rename = "webDetection", default)]
        pub web_detection:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1WebDetection>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1AnnotateImageResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1AsyncAnnotateFileRequest {
        #[doc = "Required. Requested features."]
        #[serde(rename = "features", default)]
        pub features: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P1Beta1Feature>>,
        #[doc = "Additional context that may accompany the image(s) in the file."]
        #[serde(rename = "imageContext", default)]
        pub image_context:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1ImageContext>,
        #[doc = "Required. Information about the input file."]
        #[serde(rename = "inputConfig", default)]
        pub input_config:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1InputConfig>,
        #[doc = "Required. The desired output location and metadata (e.g. format)."]
        #[serde(rename = "outputConfig", default)]
        pub output_config:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1OutputConfig>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1AsyncAnnotateFileRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1AsyncAnnotateFileResponse {
        #[doc = "The output location and metadata from AsyncAnnotateFileRequest."]
        #[serde(rename = "outputConfig", default)]
        pub output_config:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1OutputConfig>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1AsyncAnnotateFileResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1AsyncBatchAnnotateFilesRequest {
        #[doc = "Optional. Target project and location to make a call.\n\nFormat: `projects/{project-id}/locations/{location-id}`.\n\nIf no parent is specified, a region will be chosen automatically.\n\nSupported location-ids:\n`us`: USA country only,\n`asia`: East asia areas, like Japan, Taiwan,\n`eu`: The European Union.\n\nExample: `projects/project-A/locations/eu`."]
        #[serde(rename = "parent", default)]
        pub parent: ::std::option::Option<String>,
        #[doc = "Individual async file annotation requests for this batch."]
        #[serde(rename = "requests", default)]
        pub requests: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P1Beta1AsyncAnnotateFileRequest>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1AsyncBatchAnnotateFilesRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1AsyncBatchAnnotateFilesResponse {
        #[doc = "The list of file annotation responses, one for each request in\nAsyncBatchAnnotateFilesRequest."]
        #[serde(rename = "responses", default)]
        pub responses: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P1Beta1AsyncAnnotateFileResponse>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1AsyncBatchAnnotateFilesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1AsyncBatchAnnotateImagesRequest {
        #[doc = "Required. The desired output location and metadata (e.g. format)."]
        #[serde(rename = "outputConfig", default)]
        pub output_config:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1OutputConfig>,
        #[doc = "Optional. Target project and location to make a call.\n\nFormat: `projects/{project-id}/locations/{location-id}`.\n\nIf no parent is specified, a region will be chosen automatically.\n\nSupported location-ids:\n`us`: USA country only,\n`asia`: East asia areas, like Japan, Taiwan,\n`eu`: The European Union.\n\nExample: `projects/project-A/locations/eu`."]
        #[serde(rename = "parent", default)]
        pub parent: ::std::option::Option<String>,
        #[doc = "Individual image annotation requests for this batch."]
        #[serde(rename = "requests", default)]
        pub requests: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P1Beta1AnnotateImageRequest>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1AsyncBatchAnnotateImagesRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1BatchAnnotateFilesRequest {
        #[doc = "Optional. Target project and location to make a call.\n\nFormat: `projects/{project-id}/locations/{location-id}`.\n\nIf no parent is specified, a region will be chosen automatically.\n\nSupported location-ids:\n`us`: USA country only,\n`asia`: East asia areas, like Japan, Taiwan,\n`eu`: The European Union.\n\nExample: `projects/project-A/locations/eu`."]
        #[serde(rename = "parent", default)]
        pub parent: ::std::option::Option<String>,
        #[doc = "The list of file annotation requests. Right now we support only one\nAnnotateFileRequest in BatchAnnotateFilesRequest."]
        #[serde(rename = "requests", default)]
        pub requests: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P1Beta1AnnotateFileRequest>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1BatchAnnotateFilesRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVisionV1P1Beta1BatchAnnotateFilesResponse {
        #[doc = "The list of file annotation responses, each response corresponding to each\nAnnotateFileRequest in BatchAnnotateFilesRequest."]
        #[serde(rename = "responses", default)]
        pub responses: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P1Beta1AnnotateFileResponse>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1BatchAnnotateFilesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1BatchAnnotateImagesRequest {
        #[doc = "Optional. Target project and location to make a call.\n\nFormat: `projects/{project-id}/locations/{location-id}`.\n\nIf no parent is specified, a region will be chosen automatically.\n\nSupported location-ids:\n`us`: USA country only,\n`asia`: East asia areas, like Japan, Taiwan,\n`eu`: The European Union.\n\nExample: `projects/project-A/locations/eu`."]
        #[serde(rename = "parent", default)]
        pub parent: ::std::option::Option<String>,
        #[doc = "Individual image annotation requests for this batch."]
        #[serde(rename = "requests", default)]
        pub requests: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P1Beta1AnnotateImageRequest>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1BatchAnnotateImagesRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVisionV1P1Beta1BatchAnnotateImagesResponse {
        #[doc = "Individual responses to image annotation requests within the batch."]
        #[serde(rename = "responses", default)]
        pub responses: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P1Beta1AnnotateImageResponse>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1BatchAnnotateImagesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1Block {
        #[doc = "Detected block type (text, image etc) for this block."]
        #[serde(rename = "blockType", default)]
        pub block_type:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1BlockBlockType>,
        #[doc = "The bounding box for the block.\nThe vertices are in the order of top-left, top-right, bottom-right,\nbottom-left. When a rotation of the bounding box is detected the rotation\nis represented as around the top-left corner as defined when the text is\nread in the 'natural' orientation.\nFor example:\n\n* when the text is horizontal it might look like:\n  \n  ````text\n    0----1\n    |    |\n    3----2\n  ````\n\n* when it's rotated 180 degrees around the top-left corner it becomes:\n  \n  ````text\n    2----3\n    |    |\n    1----0\n  ````\n  \n  and the vertex order will still be (0, 1, 2, 3)."]
        #[serde(rename = "boundingBox", default)]
        pub bounding_box:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1BoundingPoly>,
        #[doc = "Confidence of the OCR results on the block. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "List of paragraphs in this block (if this blocks is of type text)."]
        #[serde(rename = "paragraphs", default)]
        pub paragraphs:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P1Beta1Paragraph>>,
        #[doc = "Additional information detected for the block."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P1Beta1TextAnnotationTextProperty,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1Block {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P1Beta1BlockBlockType {
        #[doc = "Barcode block."]
        Barcode,
        #[doc = "Image block."]
        Picture,
        #[doc = "Horizontal/vertical line box."]
        Ruler,
        #[doc = "Table block."]
        Table,
        #[doc = "Regular text block."]
        Text,
        #[doc = "Unknown block type."]
        Unknown,
    }
    impl GoogleCloudVisionV1P1Beta1BlockBlockType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P1Beta1BlockBlockType::Barcode => "BARCODE",
                GoogleCloudVisionV1P1Beta1BlockBlockType::Picture => "PICTURE",
                GoogleCloudVisionV1P1Beta1BlockBlockType::Ruler => "RULER",
                GoogleCloudVisionV1P1Beta1BlockBlockType::Table => "TABLE",
                GoogleCloudVisionV1P1Beta1BlockBlockType::Text => "TEXT",
                GoogleCloudVisionV1P1Beta1BlockBlockType::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P1Beta1BlockBlockType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P1Beta1BlockBlockType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P1Beta1BlockBlockType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BARCODE" => GoogleCloudVisionV1P1Beta1BlockBlockType::Barcode,
                "PICTURE" => GoogleCloudVisionV1P1Beta1BlockBlockType::Picture,
                "RULER" => GoogleCloudVisionV1P1Beta1BlockBlockType::Ruler,
                "TABLE" => GoogleCloudVisionV1P1Beta1BlockBlockType::Table,
                "TEXT" => GoogleCloudVisionV1P1Beta1BlockBlockType::Text,
                "UNKNOWN" => GoogleCloudVisionV1P1Beta1BlockBlockType::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1BlockBlockType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1BoundingPoly {
        #[doc = "The bounding polygon normalized vertices."]
        #[serde(rename = "normalizedVertices", default)]
        pub normalized_vertices:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P1Beta1NormalizedVertex>>,
        #[doc = "The bounding polygon vertices."]
        #[serde(rename = "vertices", default)]
        pub vertices: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P1Beta1Vertex>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1BoundingPoly {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1ColorInfo {
        #[doc = "RGB components of the color."]
        #[serde(rename = "color", default)]
        pub color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The fraction of pixels the color occupies in the image.\nValue in range [0, 1]."]
        #[serde(rename = "pixelFraction", default)]
        pub pixel_fraction: ::std::option::Option<f32>,
        #[doc = "Image-specific score for this color. Value in range [0, 1]."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1ColorInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1CropHint {
        #[doc = "The bounding polygon for the crop region. The coordinates of the bounding\nbox are in the original image's scale."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1BoundingPoly>,
        #[doc = "Confidence of this being a salient region.  Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Fraction of importance of this salient region with respect to the original\nimage."]
        #[serde(rename = "importanceFraction", default)]
        pub importance_fraction: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1CropHint {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1CropHintsAnnotation {
        #[doc = "Crop hint results."]
        #[serde(rename = "cropHints", default)]
        pub crop_hints:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P1Beta1CropHint>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1CropHintsAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1CropHintsParams {
        #[doc = "Aspect ratios in floats, representing the ratio of the width to the height\nof the image. For example, if the desired aspect ratio is 4/3, the\ncorresponding float value should be 1.33333.  If not specified, the\nbest possible crop is returned. The number of provided aspect ratios is\nlimited to a maximum of 16; any aspect ratios provided after the 16th are\nignored."]
        #[serde(rename = "aspectRatios", default)]
        pub aspect_ratios: ::std::option::Option<Vec<f32>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1CropHintsParams {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1DominantColorsAnnotation {
        #[doc = "RGB color values with their score and pixel fraction."]
        #[serde(rename = "colors", default)]
        pub colors: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P1Beta1ColorInfo>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1DominantColorsAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1EntityAnnotation {
        #[doc = "Image region to which this entity belongs. Not produced\nfor `LABEL_DETECTION` features."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1BoundingPoly>,
        #[doc = "**Deprecated. Use `score` instead.**\nThe accuracy of the entity detection in an image.\nFor example, for an image in which the \"Eiffel Tower\" entity is detected,\nthis field represents the confidence that there is a tower in the query\nimage. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Entity textual description, expressed in its `locale` language."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "The language code for the locale in which the entity textual\n`description` is expressed."]
        #[serde(rename = "locale", default)]
        pub locale: ::std::option::Option<String>,
        #[doc = "The location information for the detected entity. Multiple\n`LocationInfo` elements can be present because one location may\nindicate the location of the scene in the image, and another location\nmay indicate the location of the place where the image was taken.\nLocation information is usually present for landmarks."]
        #[serde(rename = "locations", default)]
        pub locations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P1Beta1LocationInfo>>,
        #[doc = "Opaque entity ID. Some IDs may be available in\n[Google Knowledge Graph Search\nAPI](https://developers.google.com/knowledge-graph/)."]
        #[serde(rename = "mid", default)]
        pub mid: ::std::option::Option<String>,
        #[doc = "Some entities may have optional user-supplied `Property` (name/value)\nfields, such a score or string that qualifies the entity."]
        #[serde(rename = "properties", default)]
        pub properties:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P1Beta1Property>>,
        #[doc = "Overall score of the result. Range [0, 1]."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
        #[doc = "The relevancy of the ICA (Image Content Annotation) label to the\nimage. For example, the relevancy of \"tower\" is likely higher to an image\ncontaining the detected \"Eiffel Tower\" than to an image containing a\ndetected distant towering building, even though the confidence that\nthere is a tower in each image may be the same. Range [0, 1]."]
        #[serde(rename = "topicality", default)]
        pub topicality: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1EntityAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1FaceAnnotation {
        #[doc = "Anger likelihood."]
        #[serde(rename = "angerLikelihood", default)]
        pub anger_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P1Beta1FaceAnnotationAngerLikelihood,
        >,
        #[doc = "Blurred likelihood."]
        #[serde(rename = "blurredLikelihood", default)]
        pub blurred_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P1Beta1FaceAnnotationBlurredLikelihood,
        >,
        #[doc = "The bounding polygon around the face. The coordinates of the bounding box\nare in the original image's scale.\nThe bounding box is computed to \"frame\" the face in accordance with human\nexpectations. It is based on the landmarker results.\nNote that one or more x and/or y coordinates may not be generated in the\n`BoundingPoly` (the polygon will be unbounded) if only a partial face\nappears in the image to be annotated."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1BoundingPoly>,
        #[doc = "Detection confidence. Range [0, 1]."]
        #[serde(rename = "detectionConfidence", default)]
        pub detection_confidence: ::std::option::Option<f32>,
        #[doc = "The `fd_bounding_poly` bounding polygon is tighter than the\n`boundingPoly`, and encloses only the skin part of the face. Typically, it\nis used to eliminate the face from any image analysis that detects the\n\"amount of skin\" visible in an image. It is not based on the\nlandmarker results, only on the initial face detection, hence\nthe <code>fd</code> (face detection) prefix."]
        #[serde(rename = "fdBoundingPoly", default)]
        pub fd_bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1BoundingPoly>,
        #[doc = "Headwear likelihood."]
        #[serde(rename = "headwearLikelihood", default)]
        pub headwear_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P1Beta1FaceAnnotationHeadwearLikelihood,
        >,
        #[doc = "Joy likelihood."]
        #[serde(rename = "joyLikelihood", default)]
        pub joy_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P1Beta1FaceAnnotationJoyLikelihood,
        >,
        #[doc = "Face landmarking confidence. Range [0, 1]."]
        #[serde(rename = "landmarkingConfidence", default)]
        pub landmarking_confidence: ::std::option::Option<f32>,
        #[doc = "Detected face landmarks."]
        #[serde(rename = "landmarks", default)]
        pub landmarks: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P1Beta1FaceAnnotationLandmark>,
        >,
        #[doc = "Yaw angle, which indicates the leftward/rightward angle that the face is\npointing relative to the vertical plane perpendicular to the image. Range\n[-180,180]."]
        #[serde(rename = "panAngle", default)]
        pub pan_angle: ::std::option::Option<f32>,
        #[doc = "Roll angle, which indicates the amount of clockwise/anti-clockwise rotation\nof the face relative to the image vertical about the axis perpendicular to\nthe face. Range [-180,180]."]
        #[serde(rename = "rollAngle", default)]
        pub roll_angle: ::std::option::Option<f32>,
        #[doc = "Sorrow likelihood."]
        #[serde(rename = "sorrowLikelihood", default)]
        pub sorrow_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P1Beta1FaceAnnotationSorrowLikelihood,
        >,
        #[doc = "Surprise likelihood."]
        #[serde(rename = "surpriseLikelihood", default)]
        pub surprise_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P1Beta1FaceAnnotationSurpriseLikelihood,
        >,
        #[doc = "Pitch angle, which indicates the upwards/downwards angle that the face is\npointing relative to the image's horizontal plane. Range [-180,180]."]
        #[serde(rename = "tiltAngle", default)]
        pub tilt_angle: ::std::option::Option<f32>,
        #[doc = "Under-exposed likelihood."]
        #[serde(rename = "underExposedLikelihood", default)]
        pub under_exposed_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P1Beta1FaceAnnotationUnderExposedLikelihood,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1FaceAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P1Beta1FaceAnnotationAngerLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P1Beta1FaceAnnotationAngerLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P1Beta1FaceAnnotationAngerLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P1Beta1FaceAnnotationAngerLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P1Beta1FaceAnnotationAngerLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P1Beta1FaceAnnotationAngerLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P1Beta1FaceAnnotationAngerLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationAngerLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P1Beta1FaceAnnotationAngerLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P1Beta1FaceAnnotationAngerLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P1Beta1FaceAnnotationAngerLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P1Beta1FaceAnnotationAngerLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P1Beta1FaceAnnotationAngerLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P1Beta1FaceAnnotationAngerLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P1Beta1FaceAnnotationAngerLikelihood::Unlikely,
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationAngerLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationAngerLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1FaceAnnotationAngerLikelihood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P1Beta1FaceAnnotationBlurredLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P1Beta1FaceAnnotationBlurredLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P1Beta1FaceAnnotationBlurredLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P1Beta1FaceAnnotationBlurredLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P1Beta1FaceAnnotationBlurredLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P1Beta1FaceAnnotationBlurredLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P1Beta1FaceAnnotationBlurredLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationBlurredLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P1Beta1FaceAnnotationBlurredLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P1Beta1FaceAnnotationBlurredLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P1Beta1FaceAnnotationBlurredLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P1Beta1FaceAnnotationBlurredLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P1Beta1FaceAnnotationBlurredLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P1Beta1FaceAnnotationBlurredLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P1Beta1FaceAnnotationBlurredLikelihood::Unlikely,
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationBlurredLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationBlurredLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1FaceAnnotationBlurredLikelihood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P1Beta1FaceAnnotationHeadwearLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P1Beta1FaceAnnotationHeadwearLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P1Beta1FaceAnnotationHeadwearLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P1Beta1FaceAnnotationHeadwearLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P1Beta1FaceAnnotationHeadwearLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P1Beta1FaceAnnotationHeadwearLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P1Beta1FaceAnnotationHeadwearLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationHeadwearLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P1Beta1FaceAnnotationHeadwearLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P1Beta1FaceAnnotationHeadwearLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P1Beta1FaceAnnotationHeadwearLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P1Beta1FaceAnnotationHeadwearLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P1Beta1FaceAnnotationHeadwearLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P1Beta1FaceAnnotationHeadwearLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P1Beta1FaceAnnotationHeadwearLikelihood::Unlikely,
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationHeadwearLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationHeadwearLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVisionV1P1Beta1FaceAnnotationHeadwearLikelihood
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P1Beta1FaceAnnotationJoyLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P1Beta1FaceAnnotationJoyLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P1Beta1FaceAnnotationJoyLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P1Beta1FaceAnnotationJoyLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P1Beta1FaceAnnotationJoyLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P1Beta1FaceAnnotationJoyLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P1Beta1FaceAnnotationJoyLikelihood::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P1Beta1FaceAnnotationJoyLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P1Beta1FaceAnnotationJoyLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P1Beta1FaceAnnotationJoyLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P1Beta1FaceAnnotationJoyLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P1Beta1FaceAnnotationJoyLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P1Beta1FaceAnnotationJoyLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P1Beta1FaceAnnotationJoyLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P1Beta1FaceAnnotationJoyLikelihood::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P1Beta1FaceAnnotationJoyLikelihood::VeryLikely,
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationJoyLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1FaceAnnotationJoyLikelihood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P1Beta1FaceAnnotationSorrowLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P1Beta1FaceAnnotationSorrowLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P1Beta1FaceAnnotationSorrowLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P1Beta1FaceAnnotationSorrowLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P1Beta1FaceAnnotationSorrowLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P1Beta1FaceAnnotationSorrowLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P1Beta1FaceAnnotationSorrowLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationSorrowLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P1Beta1FaceAnnotationSorrowLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P1Beta1FaceAnnotationSorrowLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P1Beta1FaceAnnotationSorrowLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P1Beta1FaceAnnotationSorrowLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P1Beta1FaceAnnotationSorrowLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P1Beta1FaceAnnotationSorrowLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P1Beta1FaceAnnotationSorrowLikelihood::Unlikely,
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationSorrowLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationSorrowLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1FaceAnnotationSorrowLikelihood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P1Beta1FaceAnnotationSurpriseLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P1Beta1FaceAnnotationSurpriseLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P1Beta1FaceAnnotationSurpriseLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P1Beta1FaceAnnotationSurpriseLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P1Beta1FaceAnnotationSurpriseLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P1Beta1FaceAnnotationSurpriseLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P1Beta1FaceAnnotationSurpriseLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationSurpriseLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P1Beta1FaceAnnotationSurpriseLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P1Beta1FaceAnnotationSurpriseLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P1Beta1FaceAnnotationSurpriseLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P1Beta1FaceAnnotationSurpriseLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P1Beta1FaceAnnotationSurpriseLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P1Beta1FaceAnnotationSurpriseLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P1Beta1FaceAnnotationSurpriseLikelihood::Unlikely,
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationSurpriseLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationSurpriseLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVisionV1P1Beta1FaceAnnotationSurpriseLikelihood
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P1Beta1FaceAnnotationUnderExposedLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P1Beta1FaceAnnotationUnderExposedLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P1Beta1FaceAnnotationUnderExposedLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P1Beta1FaceAnnotationUnderExposedLikelihood::Possible => {
                    "POSSIBLE"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationUnderExposedLikelihood::Unknown => {
                    "UNKNOWN"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationUnderExposedLikelihood::Unlikely => {
                    "UNLIKELY"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationUnderExposedLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationUnderExposedLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P1Beta1FaceAnnotationUnderExposedLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P1Beta1FaceAnnotationUnderExposedLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVisionV1P1Beta1FaceAnnotationUnderExposedLikelihood
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P1Beta1FaceAnnotationUnderExposedLikelihood::Likely,
                "POSSIBLE" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationUnderExposedLikelihood::Possible
                }
                "UNKNOWN" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationUnderExposedLikelihood::Unknown
                }
                "UNLIKELY" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationUnderExposedLikelihood::Unlikely
                }
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationUnderExposedLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationUnderExposedLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVisionV1P1Beta1FaceAnnotationUnderExposedLikelihood
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1FaceAnnotationLandmark {
        #[doc = "Face landmark position."]
        #[serde(rename = "position", default)]
        pub position: ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1Position>,
        #[doc = "Face landmark type."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1FaceAnnotationLandmark {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType {
        #[doc = "Chin gnathion."]
        ChinGnathion,
        #[doc = "Chin left gonion."]
        ChinLeftGonion,
        #[doc = "Chin right gonion."]
        ChinRightGonion,
        #[doc = "Forehead glabella."]
        ForeheadGlabella,
        #[doc = "Left ear tragion."]
        LeftEarTragion,
        #[doc = "Left eye."]
        LeftEye,
        #[doc = "Left eye, bottom boundary."]
        LeftEyeBottomBoundary,
        #[doc = "Left eye, left corner."]
        LeftEyeLeftCorner,
        #[doc = "Left eye pupil."]
        LeftEyePupil,
        #[doc = "Left eye, right corner."]
        LeftEyeRightCorner,
        #[doc = "Left eye, top boundary."]
        LeftEyeTopBoundary,
        #[doc = "Left eyebrow, upper midpoint."]
        LeftEyebrowUpperMidpoint,
        #[doc = "Left of left eyebrow."]
        LeftOfLeftEyebrow,
        #[doc = "Left of right eyebrow."]
        LeftOfRightEyebrow,
        #[doc = "Lower lip."]
        LowerLip,
        #[doc = "Midpoint between eyes."]
        MidpointBetweenEyes,
        #[doc = "Mouth center."]
        MouthCenter,
        #[doc = "Mouth left."]
        MouthLeft,
        #[doc = "Mouth right."]
        MouthRight,
        #[doc = "Nose, bottom center."]
        NoseBottomCenter,
        #[doc = "Nose, bottom left."]
        NoseBottomLeft,
        #[doc = "Nose, bottom right."]
        NoseBottomRight,
        #[doc = "Nose tip."]
        NoseTip,
        #[doc = "Right ear tragion."]
        RightEarTragion,
        #[doc = "Right eye."]
        RightEye,
        #[doc = "Right eye, bottom boundary."]
        RightEyeBottomBoundary,
        #[doc = "Right eye, left corner."]
        RightEyeLeftCorner,
        #[doc = "Right eye pupil."]
        RightEyePupil,
        #[doc = "Right eye, right corner."]
        RightEyeRightCorner,
        #[doc = "Right eye, top boundary."]
        RightEyeTopBoundary,
        #[doc = "Right eyebrow, upper midpoint."]
        RightEyebrowUpperMidpoint,
        #[doc = "Right of left eyebrow."]
        RightOfLeftEyebrow,
        #[doc = "Right of right eyebrow."]
        RightOfRightEyebrow,
        #[doc = "Unknown face landmark detected. Should not be filled."]
        UnknownLandmark,
        #[doc = "Upper lip."]
        UpperLip,
    }
    impl GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::ChinGnathion => {
                    "CHIN_GNATHION"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::ChinLeftGonion => {
                    "CHIN_LEFT_GONION"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::ChinRightGonion => {
                    "CHIN_RIGHT_GONION"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::ForeheadGlabella => {
                    "FOREHEAD_GLABELLA"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::LeftEarTragion => {
                    "LEFT_EAR_TRAGION"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::LeftEye => "LEFT_EYE",
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::LeftEyeBottomBoundary => {
                    "LEFT_EYE_BOTTOM_BOUNDARY"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::LeftEyeLeftCorner => {
                    "LEFT_EYE_LEFT_CORNER"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::LeftEyePupil => {
                    "LEFT_EYE_PUPIL"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::LeftEyeRightCorner => {
                    "LEFT_EYE_RIGHT_CORNER"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::LeftEyeTopBoundary => {
                    "LEFT_EYE_TOP_BOUNDARY"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::LeftEyebrowUpperMidpoint => {
                    "LEFT_EYEBROW_UPPER_MIDPOINT"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::LeftOfLeftEyebrow => {
                    "LEFT_OF_LEFT_EYEBROW"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::LeftOfRightEyebrow => {
                    "LEFT_OF_RIGHT_EYEBROW"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::LowerLip => "LOWER_LIP",
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::MidpointBetweenEyes => {
                    "MIDPOINT_BETWEEN_EYES"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::MouthCenter => "MOUTH_CENTER",
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::MouthLeft => "MOUTH_LEFT",
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::MouthRight => "MOUTH_RIGHT",
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::NoseBottomCenter => {
                    "NOSE_BOTTOM_CENTER"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::NoseBottomLeft => {
                    "NOSE_BOTTOM_LEFT"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::NoseBottomRight => {
                    "NOSE_BOTTOM_RIGHT"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::NoseTip => "NOSE_TIP",
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::RightEarTragion => {
                    "RIGHT_EAR_TRAGION"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::RightEye => "RIGHT_EYE",
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::RightEyeBottomBoundary => {
                    "RIGHT_EYE_BOTTOM_BOUNDARY"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::RightEyeLeftCorner => {
                    "RIGHT_EYE_LEFT_CORNER"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::RightEyePupil => {
                    "RIGHT_EYE_PUPIL"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::RightEyeRightCorner => {
                    "RIGHT_EYE_RIGHT_CORNER"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::RightEyeTopBoundary => {
                    "RIGHT_EYE_TOP_BOUNDARY"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::RightEyebrowUpperMidpoint => {
                    "RIGHT_EYEBROW_UPPER_MIDPOINT"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::RightOfLeftEyebrow => {
                    "RIGHT_OF_LEFT_EYEBROW"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::RightOfRightEyebrow => {
                    "RIGHT_OF_RIGHT_EYEBROW"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::UnknownLandmark => {
                    "UNKNOWN_LANDMARK"
                }
                GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::UpperLip => "UPPER_LIP",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CHIN_GNATHION" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::ChinGnathion
                }
                "CHIN_LEFT_GONION" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::ChinLeftGonion
                }
                "CHIN_RIGHT_GONION" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::ChinRightGonion
                }
                "FOREHEAD_GLABELLA" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::ForeheadGlabella
                }
                "LEFT_EAR_TRAGION" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::LeftEarTragion
                }
                "LEFT_EYE" => GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::LeftEye,
                "LEFT_EYE_BOTTOM_BOUNDARY" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::LeftEyeBottomBoundary
                }
                "LEFT_EYE_LEFT_CORNER" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::LeftEyeLeftCorner
                }
                "LEFT_EYE_PUPIL" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::LeftEyePupil
                }
                "LEFT_EYE_RIGHT_CORNER" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::LeftEyeRightCorner
                }
                "LEFT_EYE_TOP_BOUNDARY" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::LeftEyeTopBoundary
                }
                "LEFT_EYEBROW_UPPER_MIDPOINT" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::LeftEyebrowUpperMidpoint
                }
                "LEFT_OF_LEFT_EYEBROW" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::LeftOfLeftEyebrow
                }
                "LEFT_OF_RIGHT_EYEBROW" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::LeftOfRightEyebrow
                }
                "LOWER_LIP" => GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::LowerLip,
                "MIDPOINT_BETWEEN_EYES" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::MidpointBetweenEyes
                }
                "MOUTH_CENTER" => GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::MouthCenter,
                "MOUTH_LEFT" => GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::MouthLeft,
                "MOUTH_RIGHT" => GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::MouthRight,
                "NOSE_BOTTOM_CENTER" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::NoseBottomCenter
                }
                "NOSE_BOTTOM_LEFT" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::NoseBottomLeft
                }
                "NOSE_BOTTOM_RIGHT" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::NoseBottomRight
                }
                "NOSE_TIP" => GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::NoseTip,
                "RIGHT_EAR_TRAGION" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::RightEarTragion
                }
                "RIGHT_EYE" => GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::RightEye,
                "RIGHT_EYE_BOTTOM_BOUNDARY" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::RightEyeBottomBoundary
                }
                "RIGHT_EYE_LEFT_CORNER" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::RightEyeLeftCorner
                }
                "RIGHT_EYE_PUPIL" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::RightEyePupil
                }
                "RIGHT_EYE_RIGHT_CORNER" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::RightEyeRightCorner
                }
                "RIGHT_EYE_TOP_BOUNDARY" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::RightEyeTopBoundary
                }
                "RIGHT_EYEBROW_UPPER_MIDPOINT" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::RightEyebrowUpperMidpoint
                }
                "RIGHT_OF_LEFT_EYEBROW" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::RightOfLeftEyebrow
                }
                "RIGHT_OF_RIGHT_EYEBROW" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::RightOfRightEyebrow
                }
                "UNKNOWN_LANDMARK" => {
                    GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::UnknownLandmark
                }
                "UPPER_LIP" => GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType::UpperLip,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1FaceAnnotationLandmarkType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1Feature {
        #[doc = "Maximum number of results of this type. Does not apply to\n`TEXT_DETECTION`, `DOCUMENT_TEXT_DETECTION`, or `CROP_HINTS`."]
        #[serde(rename = "maxResults", default)]
        pub max_results: ::std::option::Option<i32>,
        #[doc = "Model to use for the feature.\nSupported values: \"builtin/stable\" (the default if unset) and\n\"builtin/latest\"."]
        #[serde(rename = "model", default)]
        pub model: ::std::option::Option<String>,
        #[doc = "The feature type."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1FeatureType>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1Feature {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P1Beta1FeatureType {
        #[doc = "Run crop hints."]
        CropHints,
        #[doc = "Run dense text document OCR. Takes precedence when both\n`DOCUMENT_TEXT_DETECTION` and `TEXT_DETECTION` are present."]
        DocumentTextDetection,
        #[doc = "Run face detection."]
        FaceDetection,
        #[doc = "Compute a set of image properties, such as the\nimage's dominant colors."]
        ImageProperties,
        #[doc = "Run label detection."]
        LabelDetection,
        #[doc = "Run landmark detection."]
        LandmarkDetection,
        #[doc = "Run logo detection."]
        LogoDetection,
        #[doc = "Run localizer for object detection."]
        ObjectLocalization,
        #[doc = "Run Product Search."]
        ProductSearch,
        #[doc = "Run Safe Search to detect potentially unsafe\nor undesirable content."]
        SafeSearchDetection,
        #[doc = "Run text detection / optical character recognition (OCR). Text detection\nis optimized for areas of text within a larger image; if the image is\na document, use `DOCUMENT_TEXT_DETECTION` instead."]
        TextDetection,
        #[doc = "Unspecified feature type."]
        TypeUnspecified,
        #[doc = "Run web detection."]
        WebDetection,
    }
    impl GoogleCloudVisionV1P1Beta1FeatureType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P1Beta1FeatureType::CropHints => "CROP_HINTS",
                GoogleCloudVisionV1P1Beta1FeatureType::DocumentTextDetection => {
                    "DOCUMENT_TEXT_DETECTION"
                }
                GoogleCloudVisionV1P1Beta1FeatureType::FaceDetection => "FACE_DETECTION",
                GoogleCloudVisionV1P1Beta1FeatureType::ImageProperties => "IMAGE_PROPERTIES",
                GoogleCloudVisionV1P1Beta1FeatureType::LabelDetection => "LABEL_DETECTION",
                GoogleCloudVisionV1P1Beta1FeatureType::LandmarkDetection => "LANDMARK_DETECTION",
                GoogleCloudVisionV1P1Beta1FeatureType::LogoDetection => "LOGO_DETECTION",
                GoogleCloudVisionV1P1Beta1FeatureType::ObjectLocalization => "OBJECT_LOCALIZATION",
                GoogleCloudVisionV1P1Beta1FeatureType::ProductSearch => "PRODUCT_SEARCH",
                GoogleCloudVisionV1P1Beta1FeatureType::SafeSearchDetection => {
                    "SAFE_SEARCH_DETECTION"
                }
                GoogleCloudVisionV1P1Beta1FeatureType::TextDetection => "TEXT_DETECTION",
                GoogleCloudVisionV1P1Beta1FeatureType::TypeUnspecified => "TYPE_UNSPECIFIED",
                GoogleCloudVisionV1P1Beta1FeatureType::WebDetection => "WEB_DETECTION",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P1Beta1FeatureType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P1Beta1FeatureType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P1Beta1FeatureType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CROP_HINTS" => GoogleCloudVisionV1P1Beta1FeatureType::CropHints,
                "DOCUMENT_TEXT_DETECTION" => {
                    GoogleCloudVisionV1P1Beta1FeatureType::DocumentTextDetection
                }
                "FACE_DETECTION" => GoogleCloudVisionV1P1Beta1FeatureType::FaceDetection,
                "IMAGE_PROPERTIES" => GoogleCloudVisionV1P1Beta1FeatureType::ImageProperties,
                "LABEL_DETECTION" => GoogleCloudVisionV1P1Beta1FeatureType::LabelDetection,
                "LANDMARK_DETECTION" => GoogleCloudVisionV1P1Beta1FeatureType::LandmarkDetection,
                "LOGO_DETECTION" => GoogleCloudVisionV1P1Beta1FeatureType::LogoDetection,
                "OBJECT_LOCALIZATION" => GoogleCloudVisionV1P1Beta1FeatureType::ObjectLocalization,
                "PRODUCT_SEARCH" => GoogleCloudVisionV1P1Beta1FeatureType::ProductSearch,
                "SAFE_SEARCH_DETECTION" => {
                    GoogleCloudVisionV1P1Beta1FeatureType::SafeSearchDetection
                }
                "TEXT_DETECTION" => GoogleCloudVisionV1P1Beta1FeatureType::TextDetection,
                "TYPE_UNSPECIFIED" => GoogleCloudVisionV1P1Beta1FeatureType::TypeUnspecified,
                "WEB_DETECTION" => GoogleCloudVisionV1P1Beta1FeatureType::WebDetection,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1FeatureType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1GcsDestination {
        #[doc = "Google Cloud Storage URI prefix where the results will be stored. Results\nwill be in JSON format and preceded by its corresponding input URI prefix.\nThis field can either represent a gcs file prefix or gcs directory. In\neither case, the uri should be unique because in order to get all of the\noutput files, you will need to do a wildcard gcs search on the uri prefix\nyou provide.\n\nExamples:\n\n* File Prefix: gs://bucket-name/here/filenameprefix   The output files\n  will be created in gs://bucket-name/here/ and the names of the\n  output files will begin with \"filenameprefix\".\n\n* Directory Prefix: gs://bucket-name/some/location/   The output files\n  will be created in gs://bucket-name/some/location/ and the names of the\n  output files could be anything because there was no filename prefix\n  specified.\n\nIf multiple outputs, each response is still AnnotateFileResponse, each of\nwhich contains some subset of the full list of AnnotateImageResponse.\nMultiple outputs can happen if, for example, the output JSON is too large\nand overflows into multiple sharded files."]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1GcsDestination {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1GcsSource {
        #[doc = "Google Cloud Storage URI for the input file. This must only be a\nGoogle Cloud Storage object. Wildcards are not currently supported."]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1GcsSource {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1Image {
        #[doc = "Image content, represented as a stream of bytes.\nNote: As with all `bytes` fields, protobuffers use a pure binary\nrepresentation, whereas JSON representations use base64."]
        #[serde(rename = "content", default)]
        pub content: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "Google Cloud Storage image location, or publicly-accessible image\nURL. If both `content` and `source` are provided for an image, `content`\ntakes precedence and is used to perform the image annotation request."]
        #[serde(rename = "source", default)]
        pub source: ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1ImageSource>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1Image {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1ImageAnnotationContext {
        #[doc = "If the file was a PDF or TIFF, this field gives the page number within\nthe file used to produce the image."]
        #[serde(rename = "pageNumber", default)]
        pub page_number: ::std::option::Option<i32>,
        #[doc = "The URI of the file used to produce the image."]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1ImageAnnotationContext {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1ImageContext {
        #[doc = "Parameters for crop hints annotation request."]
        #[serde(rename = "cropHintsParams", default)]
        pub crop_hints_params:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1CropHintsParams>,
        #[doc = "List of languages to use for TEXT_DETECTION. In most cases, an empty value\nyields the best results since it enables automatic language detection. For\nlanguages based on the Latin alphabet, setting `language_hints` is not\nneeded. In rare cases, when the language of the text in the image is known,\nsetting a hint will help get better results (although it will be a\nsignificant hindrance if the hint is wrong). Text detection returns an\nerror if one or more of the specified languages is not one of the\n[supported languages](/vision/docs/languages)."]
        #[serde(rename = "languageHints", default)]
        pub language_hints: ::std::option::Option<Vec<String>>,
        #[doc = "Not used."]
        #[serde(rename = "latLongRect", default)]
        pub lat_long_rect:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1LatLongRect>,
        #[doc = "Parameters for product search."]
        #[serde(rename = "productSearchParams", default)]
        pub product_search_params:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1ProductSearchParams>,
        #[doc = "Parameters for web detection."]
        #[serde(rename = "webDetectionParams", default)]
        pub web_detection_params:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1WebDetectionParams>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1ImageContext {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1ImageProperties {
        #[doc = "If present, dominant colors completed successfully."]
        #[serde(rename = "dominantColors", default)]
        pub dominant_colors: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P1Beta1DominantColorsAnnotation,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1ImageProperties {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1ImageSource {
        #[doc = "**Use `image_uri` instead.**\n\nThe Google Cloud Storage  URI of the form\n`gs://bucket_name/object_name`. Object versioning is not supported. See\n[Google Cloud Storage Request\nURIs](https://cloud.google.com/storage/docs/reference-uris) for more info."]
        #[serde(rename = "gcsImageUri", default)]
        pub gcs_image_uri: ::std::option::Option<String>,
        #[doc = "The URI of the source image. Can be either:\n\n1. A Google Cloud Storage URI of the form\n   `gs://bucket_name/object_name`. Object versioning is not supported. See\n   [Google Cloud Storage Request\n   URIs](https://cloud.google.com/storage/docs/reference-uris) for more\n   info.\n\n1. A publicly-accessible image HTTP/HTTPS URL. When fetching images from\n   HTTP/HTTPS URLs, Google cannot guarantee that the request will be\n   completed. Your request may fail if the specified host denies the\n   request (e.g. due to request throttling or DOS prevention), or if Google\n   throttles requests to the site for abuse prevention. You should not\n   depend on externally-hosted images for production applications.\n\nWhen both `gcs_image_uri` and `image_uri` are specified, `image_uri` takes\nprecedence."]
        #[serde(rename = "imageUri", default)]
        pub image_uri: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1ImageSource {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1InputConfig {
        #[doc = "File content, represented as a stream of bytes.\nNote: As with all `bytes` fields, protobuffers use a pure binary\nrepresentation, whereas JSON representations use base64.\n\nCurrently, this field only works for BatchAnnotateFiles requests. It does\nnot work for AsyncBatchAnnotateFiles requests."]
        #[serde(rename = "content", default)]
        pub content: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "The Google Cloud Storage location to read the input from."]
        #[serde(rename = "gcsSource", default)]
        pub gcs_source: ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1GcsSource>,
        #[doc = "The type of the file. Currently only \"application/pdf\", \"image/tiff\" and\n\"image/gif\" are supported. Wildcards are not supported."]
        #[serde(rename = "mimeType", default)]
        pub mime_type: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1InputConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1LatLongRect {
        #[doc = "Max lat/long pair."]
        #[serde(rename = "maxLatLng", default)]
        pub max_lat_lng: ::std::option::Option<crate::schemas::LatLng>,
        #[doc = "Min lat/long pair."]
        #[serde(rename = "minLatLng", default)]
        pub min_lat_lng: ::std::option::Option<crate::schemas::LatLng>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1LatLongRect {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1LocalizedObjectAnnotation {
        #[doc = "Image region to which this object belongs. This must be populated."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1BoundingPoly>,
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more\ninformation, see\nhttp://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Object ID that should align with EntityAnnotation mid."]
        #[serde(rename = "mid", default)]
        pub mid: ::std::option::Option<String>,
        #[doc = "Object name, expressed in its `language_code` language."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Score of the result. Range [0, 1]."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1LocalizedObjectAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1LocationInfo {
        #[doc = "lat/long location coordinates."]
        #[serde(rename = "latLng", default)]
        pub lat_lng: ::std::option::Option<crate::schemas::LatLng>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1LocationInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1NormalizedVertex {
        #[doc = "X coordinate."]
        #[serde(rename = "x", default)]
        pub x: ::std::option::Option<f32>,
        #[doc = "Y coordinate."]
        #[serde(rename = "y", default)]
        pub y: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1NormalizedVertex {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1OperationMetadata {
        #[doc = "The time when the batch request was received."]
        #[serde(rename = "createTime", default)]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Current state of the batch operation."]
        #[serde(rename = "state", default)]
        pub state:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1OperationMetadataState>,
        #[doc = "The time when the operation result was last updated."]
        #[serde(rename = "updateTime", default)]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1OperationMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P1Beta1OperationMetadataState {
        #[doc = "The batch processing was cancelled."]
        Cancelled,
        #[doc = "Request is received."]
        Created,
        #[doc = "The batch processing is done."]
        Done,
        #[doc = "Request is actively being processed."]
        Running,
        #[doc = "Invalid."]
        StateUnspecified,
    }
    impl GoogleCloudVisionV1P1Beta1OperationMetadataState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P1Beta1OperationMetadataState::Cancelled => "CANCELLED",
                GoogleCloudVisionV1P1Beta1OperationMetadataState::Created => "CREATED",
                GoogleCloudVisionV1P1Beta1OperationMetadataState::Done => "DONE",
                GoogleCloudVisionV1P1Beta1OperationMetadataState::Running => "RUNNING",
                GoogleCloudVisionV1P1Beta1OperationMetadataState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P1Beta1OperationMetadataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P1Beta1OperationMetadataState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P1Beta1OperationMetadataState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => GoogleCloudVisionV1P1Beta1OperationMetadataState::Cancelled,
                "CREATED" => GoogleCloudVisionV1P1Beta1OperationMetadataState::Created,
                "DONE" => GoogleCloudVisionV1P1Beta1OperationMetadataState::Done,
                "RUNNING" => GoogleCloudVisionV1P1Beta1OperationMetadataState::Running,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudVisionV1P1Beta1OperationMetadataState::StateUnspecified
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1OperationMetadataState {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1OutputConfig {
        #[doc = "The max number of response protos to put into each output JSON file on\nGoogle Cloud Storage.\nThe valid range is [1, 100]. If not specified, the default value is 20.\n\nFor example, for one pdf file with 100 pages, 100 response protos will\nbe generated. If `batch_size` = 20, then 5 json files each\ncontaining 20 response protos will be written under the prefix\n`gcs_destination`.`uri`.\n\nCurrently, batch_size only applies to GcsDestination, with potential future\nsupport for other output configurations."]
        #[serde(rename = "batchSize", default)]
        pub batch_size: ::std::option::Option<i32>,
        #[doc = "The Google Cloud Storage location to write the output(s) to."]
        #[serde(rename = "gcsDestination", default)]
        pub gcs_destination:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1GcsDestination>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1OutputConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1Page {
        #[doc = "List of blocks of text, images etc on this page."]
        #[serde(rename = "blocks", default)]
        pub blocks: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P1Beta1Block>>,
        #[doc = "Confidence of the OCR results on the page. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Page height. For PDFs the unit is points. For images (including\nTIFFs) the unit is pixels."]
        #[serde(rename = "height", default)]
        pub height: ::std::option::Option<i32>,
        #[doc = "Additional information detected on the page."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P1Beta1TextAnnotationTextProperty,
        >,
        #[doc = "Page width. For PDFs the unit is points. For images (including\nTIFFs) the unit is pixels."]
        #[serde(rename = "width", default)]
        pub width: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1Page {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1Paragraph {
        #[doc = "The bounding box for the paragraph.\nThe vertices are in the order of top-left, top-right, bottom-right,\nbottom-left. When a rotation of the bounding box is detected the rotation\nis represented as around the top-left corner as defined when the text is\nread in the 'natural' orientation.\nFor example:\n\n* when the text is horizontal it might look like:\n  0----1\n  |    |\n  3----2\n* when it's rotated 180 degrees around the top-left corner it becomes:\n  2----3\n  |    |\n  1----0\n  and the vertex order will still be (0, 1, 2, 3)."]
        #[serde(rename = "boundingBox", default)]
        pub bounding_box:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1BoundingPoly>,
        #[doc = "Confidence of the OCR results for the paragraph. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Additional information detected for the paragraph."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P1Beta1TextAnnotationTextProperty,
        >,
        #[doc = "List of all words in this paragraph."]
        #[serde(rename = "words", default)]
        pub words: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P1Beta1Word>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1Paragraph {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1Position {
        #[doc = "X coordinate."]
        #[serde(rename = "x", default)]
        pub x: ::std::option::Option<f32>,
        #[doc = "Y coordinate."]
        #[serde(rename = "y", default)]
        pub y: ::std::option::Option<f32>,
        #[doc = "Z coordinate (or depth)."]
        #[serde(rename = "z", default)]
        pub z: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1Position {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1Product {
        #[doc = "User-provided metadata to be stored with this product. Must be at most 4096\ncharacters long."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "The user-provided name for this Product. Must not be empty. Must be at most\n4096 characters long."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The resource name of the product.\n\nFormat is:\n`projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`.\n\nThis field is ignored when creating a product."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The category for the product identified by the reference image. This should\nbe either \"homegoods-v2\", \"apparel-v2\", or \"toys-v2\". The legacy categories\n\"homegoods\", \"apparel\", and \"toys\" are still supported, but these should\nnot be used for new products.\n\nThis field is immutable."]
        #[serde(rename = "productCategory", default)]
        pub product_category: ::std::option::Option<String>,
        #[doc = "Key-value pairs that can be attached to a product. At query time,\nconstraints can be specified based on the product_labels.\n\nNote that integer values can be provided as strings, e.g. \"1199\". Only\nstrings with integer values can match a range-based restriction which is\nto be supported soon.\n\nMultiple values can be assigned to the same key. One product may have up to\n500 product_labels.\n\nNotice that the total number of distinct product_labels over all products\nin one ProductSet cannot exceed 1M, otherwise the product search pipeline\nwill refuse to work for that ProductSet."]
        #[serde(rename = "productLabels", default)]
        pub product_labels:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P1Beta1ProductKeyValue>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1Product {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1ProductKeyValue {
        #[doc = "The key of the label attached to the product. Cannot be empty and cannot\nexceed 128 bytes."]
        #[serde(rename = "key", default)]
        pub key: ::std::option::Option<String>,
        #[doc = "The value of the label attached to the product. Cannot be empty and\ncannot exceed 128 bytes."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1ProductKeyValue {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1ProductSearchParams {
        #[doc = "The bounding polygon around the area of interest in the image.\nOptional. If it is not specified, system discretion will be applied."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1BoundingPoly>,
        #[doc = "The filtering expression. This can be used to restrict search results based\non Product labels. We currently support an AND of OR of key-value\nexpressions, where each expression within an OR must have the same key. An\n'=' should be used to connect the key and value.\n\nFor example, \"(color = red OR color = blue) AND brand = Google\" is\nacceptable, but \"(color = red OR brand = Google)\" is not acceptable.\n\"color: red\" is not acceptable because it uses a ':' instead of an '='."]
        #[serde(rename = "filter", default)]
        pub filter: ::std::option::Option<String>,
        #[doc = "The list of product categories to search in. Currently, we only consider\nthe first category, and either \"homegoods-v2\", \"apparel-v2\", or \"toys-v2\"\nshould be specified. The legacy categories \"homegoods\", \"apparel\", and\n\"toys\" are still supported but will be deprecated. For new products, please\nuse \"homegoods-v2\", \"apparel-v2\", or \"toys-v2\" for better product search\naccuracy. It is recommended to migrate existing products to these\ncategories as well."]
        #[serde(rename = "productCategories", default)]
        pub product_categories: ::std::option::Option<Vec<String>>,
        #[doc = "The resource name of a ProductSet to be searched for similar images.\n\nFormat is:\n`projects/PROJECT_ID/locations/LOC_ID/productSets/PRODUCT_SET_ID`."]
        #[serde(rename = "productSet", default)]
        pub product_set: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1ProductSearchParams {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1ProductSearchResults {
        #[doc = "Timestamp of the index which provided these results. Products added to the\nproduct set and products removed from the product set after this time are\nnot reflected in the current results."]
        #[serde(rename = "indexTime", default)]
        pub index_time: ::std::option::Option<String>,
        #[doc = "List of results grouped by products detected in the query image. Each entry\ncorresponds to one bounding polygon in the query image, and contains the\nmatching products specific to that region. There may be duplicate product\nmatches in the union of all the per-product results."]
        #[serde(rename = "productGroupedResults", default)]
        pub product_grouped_results: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P1Beta1ProductSearchResultsGroupedResult>,
        >,
        #[doc = "List of results, one for each product match."]
        #[serde(rename = "results", default)]
        pub results: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P1Beta1ProductSearchResultsResult>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1ProductSearchResults {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1ProductSearchResultsGroupedResult {
        #[doc = "The bounding polygon around the product detected in the query image."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1BoundingPoly>,
        #[doc = "List of results, one for each product match."]
        #[serde(rename = "results", default)]
        pub results: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P1Beta1ProductSearchResultsResult>,
        >,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVisionV1P1Beta1ProductSearchResultsGroupedResult
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1ProductSearchResultsResult {
        #[doc = "The resource name of the image from the product that is the closest match\nto the query."]
        #[serde(rename = "image", default)]
        pub image: ::std::option::Option<String>,
        #[doc = "The Product."]
        #[serde(rename = "product", default)]
        pub product: ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1Product>,
        #[doc = "A confidence level on the match, ranging from 0 (no confidence) to\n1 (full confidence)."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1ProductSearchResultsResult {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1Property {
        #[doc = "Name of the property."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Value of numeric properties."]
        #[serde(rename = "uint64Value", default)]
        #[serde(with = "crate::parsed_string")]
        pub uint_64_value: ::std::option::Option<u64>,
        #[doc = "Value of the property."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1Property {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1SafeSearchAnnotation {
        #[doc = "Represents the adult content likelihood for the image. Adult content may\ncontain elements such as nudity, pornographic images or cartoons, or\nsexual activities."]
        #[serde(rename = "adult", default)]
        pub adult: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P1Beta1SafeSearchAnnotationAdult,
        >,
        #[doc = "Likelihood that this is a medical image."]
        #[serde(rename = "medical", default)]
        pub medical: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P1Beta1SafeSearchAnnotationMedical,
        >,
        #[doc = "Likelihood that the request image contains racy content. Racy content may\ninclude (but is not limited to) skimpy or sheer clothing, strategically\ncovered nudity, lewd or provocative poses, or close-ups of sensitive\nbody areas."]
        #[serde(rename = "racy", default)]
        pub racy: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P1Beta1SafeSearchAnnotationRacy,
        >,
        #[doc = "Spoof likelihood. The likelihood that an modification\nwas made to the image's canonical version to make it appear\nfunny or offensive."]
        #[serde(rename = "spoof", default)]
        pub spoof: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P1Beta1SafeSearchAnnotationSpoof,
        >,
        #[doc = "Likelihood that this image contains violent content."]
        #[serde(rename = "violence", default)]
        pub violence: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P1Beta1SafeSearchAnnotationViolence,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1SafeSearchAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P1Beta1SafeSearchAnnotationAdult {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P1Beta1SafeSearchAnnotationAdult {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationAdult::Likely => "LIKELY",
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationAdult::Possible => "POSSIBLE",
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationAdult::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationAdult::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationAdult::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationAdult::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P1Beta1SafeSearchAnnotationAdult {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P1Beta1SafeSearchAnnotationAdult {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P1Beta1SafeSearchAnnotationAdult {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P1Beta1SafeSearchAnnotationAdult::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P1Beta1SafeSearchAnnotationAdult::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P1Beta1SafeSearchAnnotationAdult::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P1Beta1SafeSearchAnnotationAdult::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P1Beta1SafeSearchAnnotationAdult::VeryLikely,
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P1Beta1SafeSearchAnnotationAdult::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1SafeSearchAnnotationAdult {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P1Beta1SafeSearchAnnotationMedical {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P1Beta1SafeSearchAnnotationMedical {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationMedical::Likely => "LIKELY",
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationMedical::Possible => "POSSIBLE",
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationMedical::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationMedical::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationMedical::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationMedical::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P1Beta1SafeSearchAnnotationMedical {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P1Beta1SafeSearchAnnotationMedical {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P1Beta1SafeSearchAnnotationMedical {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P1Beta1SafeSearchAnnotationMedical::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P1Beta1SafeSearchAnnotationMedical::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P1Beta1SafeSearchAnnotationMedical::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P1Beta1SafeSearchAnnotationMedical::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P1Beta1SafeSearchAnnotationMedical::VeryLikely,
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P1Beta1SafeSearchAnnotationMedical::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1SafeSearchAnnotationMedical {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P1Beta1SafeSearchAnnotationRacy {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P1Beta1SafeSearchAnnotationRacy {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationRacy::Likely => "LIKELY",
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationRacy::Possible => "POSSIBLE",
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationRacy::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationRacy::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationRacy::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationRacy::VeryUnlikely => "VERY_UNLIKELY",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P1Beta1SafeSearchAnnotationRacy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P1Beta1SafeSearchAnnotationRacy {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P1Beta1SafeSearchAnnotationRacy {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P1Beta1SafeSearchAnnotationRacy::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P1Beta1SafeSearchAnnotationRacy::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P1Beta1SafeSearchAnnotationRacy::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P1Beta1SafeSearchAnnotationRacy::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P1Beta1SafeSearchAnnotationRacy::VeryLikely,
                "VERY_UNLIKELY" => GoogleCloudVisionV1P1Beta1SafeSearchAnnotationRacy::VeryUnlikely,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1SafeSearchAnnotationRacy {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P1Beta1SafeSearchAnnotationSpoof {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P1Beta1SafeSearchAnnotationSpoof {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationSpoof::Likely => "LIKELY",
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationSpoof::Possible => "POSSIBLE",
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationSpoof::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationSpoof::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationSpoof::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationSpoof::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P1Beta1SafeSearchAnnotationSpoof {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P1Beta1SafeSearchAnnotationSpoof {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P1Beta1SafeSearchAnnotationSpoof {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P1Beta1SafeSearchAnnotationSpoof::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P1Beta1SafeSearchAnnotationSpoof::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P1Beta1SafeSearchAnnotationSpoof::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P1Beta1SafeSearchAnnotationSpoof::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P1Beta1SafeSearchAnnotationSpoof::VeryLikely,
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P1Beta1SafeSearchAnnotationSpoof::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1SafeSearchAnnotationSpoof {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P1Beta1SafeSearchAnnotationViolence {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P1Beta1SafeSearchAnnotationViolence {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationViolence::Likely => "LIKELY",
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationViolence::Possible => "POSSIBLE",
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationViolence::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationViolence::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationViolence::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P1Beta1SafeSearchAnnotationViolence::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P1Beta1SafeSearchAnnotationViolence {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P1Beta1SafeSearchAnnotationViolence {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P1Beta1SafeSearchAnnotationViolence {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P1Beta1SafeSearchAnnotationViolence::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P1Beta1SafeSearchAnnotationViolence::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P1Beta1SafeSearchAnnotationViolence::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P1Beta1SafeSearchAnnotationViolence::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P1Beta1SafeSearchAnnotationViolence::VeryLikely,
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P1Beta1SafeSearchAnnotationViolence::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1SafeSearchAnnotationViolence {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1Symbol {
        #[doc = "The bounding box for the symbol.\nThe vertices are in the order of top-left, top-right, bottom-right,\nbottom-left. When a rotation of the bounding box is detected the rotation\nis represented as around the top-left corner as defined when the text is\nread in the 'natural' orientation.\nFor example:\n\n* when the text is horizontal it might look like:\n  0----1\n  |    |\n  3----2\n* when it's rotated 180 degrees around the top-left corner it becomes:\n  2----3\n  |    |\n  1----0\n  and the vertex order will still be (0, 1, 2, 3)."]
        #[serde(rename = "boundingBox", default)]
        pub bounding_box:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1BoundingPoly>,
        #[doc = "Confidence of the OCR results for the symbol. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Additional information detected for the symbol."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P1Beta1TextAnnotationTextProperty,
        >,
        #[doc = "The actual UTF-8 representation of the symbol."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1Symbol {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1TextAnnotation {
        #[doc = "List of pages detected by OCR."]
        #[serde(rename = "pages", default)]
        pub pages: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P1Beta1Page>>,
        #[doc = "UTF-8 text detected on the pages."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1TextAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1TextAnnotationDetectedBreak {
        #[doc = "True if break prepends the element."]
        #[serde(rename = "isPrefix", default)]
        pub is_prefix: ::std::option::Option<bool>,
        #[doc = "Detected break type."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P1Beta1TextAnnotationDetectedBreakType,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1TextAnnotationDetectedBreak {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P1Beta1TextAnnotationDetectedBreakType {
        #[doc = "Line-wrapping break."]
        EolSureSpace,
        #[doc = "End-line hyphen that is not present in text; does not co-occur with\n`SPACE`, `LEADER_SPACE`, or `LINE_BREAK`."]
        Hyphen,
        #[doc = "Line break that ends a paragraph."]
        LineBreak,
        #[doc = "Regular space."]
        Space,
        #[doc = "Sure space (very wide)."]
        SureSpace,
        #[doc = "Unknown break label type."]
        Unknown,
    }
    impl GoogleCloudVisionV1P1Beta1TextAnnotationDetectedBreakType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P1Beta1TextAnnotationDetectedBreakType::EolSureSpace => {
                    "EOL_SURE_SPACE"
                }
                GoogleCloudVisionV1P1Beta1TextAnnotationDetectedBreakType::Hyphen => "HYPHEN",
                GoogleCloudVisionV1P1Beta1TextAnnotationDetectedBreakType::LineBreak => {
                    "LINE_BREAK"
                }
                GoogleCloudVisionV1P1Beta1TextAnnotationDetectedBreakType::Space => "SPACE",
                GoogleCloudVisionV1P1Beta1TextAnnotationDetectedBreakType::SureSpace => {
                    "SURE_SPACE"
                }
                GoogleCloudVisionV1P1Beta1TextAnnotationDetectedBreakType::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P1Beta1TextAnnotationDetectedBreakType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P1Beta1TextAnnotationDetectedBreakType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P1Beta1TextAnnotationDetectedBreakType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EOL_SURE_SPACE" => {
                    GoogleCloudVisionV1P1Beta1TextAnnotationDetectedBreakType::EolSureSpace
                }
                "HYPHEN" => GoogleCloudVisionV1P1Beta1TextAnnotationDetectedBreakType::Hyphen,
                "LINE_BREAK" => {
                    GoogleCloudVisionV1P1Beta1TextAnnotationDetectedBreakType::LineBreak
                }
                "SPACE" => GoogleCloudVisionV1P1Beta1TextAnnotationDetectedBreakType::Space,
                "SURE_SPACE" => {
                    GoogleCloudVisionV1P1Beta1TextAnnotationDetectedBreakType::SureSpace
                }
                "UNKNOWN" => GoogleCloudVisionV1P1Beta1TextAnnotationDetectedBreakType::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1TextAnnotationDetectedBreakType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1TextAnnotationDetectedLanguage {
        #[doc = "Confidence of detected language. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more\ninformation, see\nhttp://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1TextAnnotationDetectedLanguage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1TextAnnotationTextProperty {
        #[doc = "Detected start or end of a text segment."]
        #[serde(rename = "detectedBreak", default)]
        pub detected_break: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P1Beta1TextAnnotationDetectedBreak,
        >,
        #[doc = "A list of detected languages together with confidence."]
        #[serde(rename = "detectedLanguages", default)]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P1Beta1TextAnnotationDetectedLanguage>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1TextAnnotationTextProperty {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1Vertex {
        #[doc = "X coordinate."]
        #[serde(rename = "x", default)]
        pub x: ::std::option::Option<i32>,
        #[doc = "Y coordinate."]
        #[serde(rename = "y", default)]
        pub y: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1Vertex {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1WebDetection {
        #[doc = "The service's best guess as to the topic of the request image.\nInferred from similar images on the open web."]
        #[serde(rename = "bestGuessLabels", default)]
        pub best_guess_labels: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P1Beta1WebDetectionWebLabel>,
        >,
        #[doc = "Fully matching images from the Internet.\nCan include resized copies of the query image."]
        #[serde(rename = "fullMatchingImages", default)]
        pub full_matching_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P1Beta1WebDetectionWebImage>,
        >,
        #[doc = "Web pages containing the matching images from the Internet."]
        #[serde(rename = "pagesWithMatchingImages", default)]
        pub pages_with_matching_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P1Beta1WebDetectionWebPage>,
        >,
        #[doc = "Partial matching images from the Internet.\nThose images are similar enough to share some key-point features. For\nexample an original image will likely have partial matching for its crops."]
        #[serde(rename = "partialMatchingImages", default)]
        pub partial_matching_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P1Beta1WebDetectionWebImage>,
        >,
        #[doc = "The visually similar image results."]
        #[serde(rename = "visuallySimilarImages", default)]
        pub visually_similar_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P1Beta1WebDetectionWebImage>,
        >,
        #[doc = "Deduced entities from similar images on the Internet."]
        #[serde(rename = "webEntities", default)]
        pub web_entities: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P1Beta1WebDetectionWebEntity>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1WebDetection {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1WebDetectionParams {
        #[doc = "Whether to include results derived from the geo information in the image."]
        #[serde(rename = "includeGeoResults", default)]
        pub include_geo_results: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1WebDetectionParams {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1WebDetectionWebEntity {
        #[doc = "Canonical description of the entity, in English."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Opaque entity ID."]
        #[serde(rename = "entityId", default)]
        pub entity_id: ::std::option::Option<String>,
        #[doc = "Overall relevancy score for the entity.\nNot normalized and not comparable across different image queries."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1WebDetectionWebEntity {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1WebDetectionWebImage {
        #[doc = "(Deprecated) Overall relevancy score for the image."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
        #[doc = "The result image URL."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1WebDetectionWebImage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1WebDetectionWebLabel {
        #[doc = "Label for extra metadata."]
        #[serde(rename = "label", default)]
        pub label: ::std::option::Option<String>,
        #[doc = "The BCP-47 language code for `label`, such as \"en-US\" or \"sr-Latn\".\nFor more information, see\nhttp://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1WebDetectionWebLabel {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1WebDetectionWebPage {
        #[doc = "Fully matching images on the page.\nCan include resized copies of the query image."]
        #[serde(rename = "fullMatchingImages", default)]
        pub full_matching_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P1Beta1WebDetectionWebImage>,
        >,
        #[doc = "Title for the web page, may contain HTML markups."]
        #[serde(rename = "pageTitle", default)]
        pub page_title: ::std::option::Option<String>,
        #[doc = "Partial matching images on the page.\nThose images are similar enough to share some key-point features. For\nexample an original image will likely have partial matching for its\ncrops."]
        #[serde(rename = "partialMatchingImages", default)]
        pub partial_matching_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P1Beta1WebDetectionWebImage>,
        >,
        #[doc = "(Deprecated) Overall relevancy score for the web page."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
        #[doc = "The result web page URL."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1WebDetectionWebPage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P1Beta1Word {
        #[doc = "The bounding box for the word.\nThe vertices are in the order of top-left, top-right, bottom-right,\nbottom-left. When a rotation of the bounding box is detected the rotation\nis represented as around the top-left corner as defined when the text is\nread in the 'natural' orientation.\nFor example:\n\n* when the text is horizontal it might look like:\n  0----1\n  |    |\n  3----2\n* when it's rotated 180 degrees around the top-left corner it becomes:\n  2----3\n  |    |\n  1----0\n  and the vertex order will still be (0, 1, 2, 3)."]
        #[serde(rename = "boundingBox", default)]
        pub bounding_box:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P1Beta1BoundingPoly>,
        #[doc = "Confidence of the OCR results for the word. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Additional information detected for the word."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P1Beta1TextAnnotationTextProperty,
        >,
        #[doc = "List of symbols in the word.\nThe order of the symbols follows the natural reading order."]
        #[serde(rename = "symbols", default)]
        pub symbols: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P1Beta1Symbol>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P1Beta1Word {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVisionV1P2Beta1AnnotateFileResponse {
        #[doc = "Information about the file for which this response is generated."]
        #[serde(rename = "inputConfig", default)]
        pub input_config:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P2Beta1InputConfig>,
        #[doc = "Individual responses to images found within the file. This field will be\nempty if the `error` field is set."]
        #[serde(rename = "responses", default)]
        pub responses: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P2Beta1AnnotateImageResponse>,
        >,
        #[doc = "This field gives the total number of pages in the file."]
        #[serde(rename = "totalPages", default)]
        pub total_pages: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1AnnotateFileResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVisionV1P2Beta1AnnotateImageResponse {
        #[doc = "If present, contextual information is needed to understand where this image\ncomes from."]
        #[serde(rename = "context", default)]
        pub context:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P2Beta1ImageAnnotationContext>,
        #[doc = "If present, crop hints have completed successfully."]
        #[serde(rename = "cropHintsAnnotation", default)]
        pub crop_hints_annotation:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P2Beta1CropHintsAnnotation>,
        #[doc = "If set, represents the error message for the operation.\nNote that filled-in image annotations are guaranteed to be\ncorrect, even when `error` is set."]
        #[serde(rename = "error", default)]
        pub error: ::std::option::Option<crate::schemas::Status>,
        #[doc = "If present, face detection has completed successfully."]
        #[serde(rename = "faceAnnotations", default)]
        pub face_annotations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P2Beta1FaceAnnotation>>,
        #[doc = "If present, text (OCR) detection or document (OCR) text detection has\ncompleted successfully.\nThis annotation provides the structural hierarchy for the OCR detected\ntext."]
        #[serde(rename = "fullTextAnnotation", default)]
        pub full_text_annotation:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P2Beta1TextAnnotation>,
        #[doc = "If present, image properties were extracted successfully."]
        #[serde(rename = "imagePropertiesAnnotation", default)]
        pub image_properties_annotation:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P2Beta1ImageProperties>,
        #[doc = "If present, label detection has completed successfully."]
        #[serde(rename = "labelAnnotations", default)]
        pub label_annotations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P2Beta1EntityAnnotation>>,
        #[doc = "If present, landmark detection has completed successfully."]
        #[serde(rename = "landmarkAnnotations", default)]
        pub landmark_annotations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P2Beta1EntityAnnotation>>,
        #[doc = "If present, localized object detection has completed successfully.\nThis will be sorted descending by confidence score."]
        #[serde(rename = "localizedObjectAnnotations", default)]
        pub localized_object_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P2Beta1LocalizedObjectAnnotation>,
        >,
        #[doc = "If present, logo detection has completed successfully."]
        #[serde(rename = "logoAnnotations", default)]
        pub logo_annotations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P2Beta1EntityAnnotation>>,
        #[doc = "If present, product search has completed successfully."]
        #[serde(rename = "productSearchResults", default)]
        pub product_search_results:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P2Beta1ProductSearchResults>,
        #[doc = "If present, safe-search annotation has completed successfully."]
        #[serde(rename = "safeSearchAnnotation", default)]
        pub safe_search_annotation:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P2Beta1SafeSearchAnnotation>,
        #[doc = "If present, text (OCR) detection has completed successfully."]
        #[serde(rename = "textAnnotations", default)]
        pub text_annotations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P2Beta1EntityAnnotation>>,
        #[doc = "If present, web detection has completed successfully."]
        #[serde(rename = "webDetection", default)]
        pub web_detection:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P2Beta1WebDetection>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1AnnotateImageResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1AsyncAnnotateFileResponse {
        #[doc = "The output location and metadata from AsyncAnnotateFileRequest."]
        #[serde(rename = "outputConfig", default)]
        pub output_config:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P2Beta1OutputConfig>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1AsyncAnnotateFileResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1AsyncBatchAnnotateFilesResponse {
        #[doc = "The list of file annotation responses, one for each request in\nAsyncBatchAnnotateFilesRequest."]
        #[serde(rename = "responses", default)]
        pub responses: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P2Beta1AsyncAnnotateFileResponse>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1AsyncBatchAnnotateFilesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1Block {
        #[doc = "Detected block type (text, image etc) for this block."]
        #[serde(rename = "blockType", default)]
        pub block_type:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P2Beta1BlockBlockType>,
        #[doc = "The bounding box for the block.\nThe vertices are in the order of top-left, top-right, bottom-right,\nbottom-left. When a rotation of the bounding box is detected the rotation\nis represented as around the top-left corner as defined when the text is\nread in the 'natural' orientation.\nFor example:\n\n* when the text is horizontal it might look like:\n  \n  ````text\n    0----1\n    |    |\n    3----2\n  ````\n\n* when it's rotated 180 degrees around the top-left corner it becomes:\n  \n  ````text\n    2----3\n    |    |\n    1----0\n  ````\n  \n  and the vertex order will still be (0, 1, 2, 3)."]
        #[serde(rename = "boundingBox", default)]
        pub bounding_box:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P2Beta1BoundingPoly>,
        #[doc = "Confidence of the OCR results on the block. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "List of paragraphs in this block (if this blocks is of type text)."]
        #[serde(rename = "paragraphs", default)]
        pub paragraphs:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P2Beta1Paragraph>>,
        #[doc = "Additional information detected for the block."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P2Beta1TextAnnotationTextProperty,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1Block {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P2Beta1BlockBlockType {
        #[doc = "Barcode block."]
        Barcode,
        #[doc = "Image block."]
        Picture,
        #[doc = "Horizontal/vertical line box."]
        Ruler,
        #[doc = "Table block."]
        Table,
        #[doc = "Regular text block."]
        Text,
        #[doc = "Unknown block type."]
        Unknown,
    }
    impl GoogleCloudVisionV1P2Beta1BlockBlockType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P2Beta1BlockBlockType::Barcode => "BARCODE",
                GoogleCloudVisionV1P2Beta1BlockBlockType::Picture => "PICTURE",
                GoogleCloudVisionV1P2Beta1BlockBlockType::Ruler => "RULER",
                GoogleCloudVisionV1P2Beta1BlockBlockType::Table => "TABLE",
                GoogleCloudVisionV1P2Beta1BlockBlockType::Text => "TEXT",
                GoogleCloudVisionV1P2Beta1BlockBlockType::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P2Beta1BlockBlockType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P2Beta1BlockBlockType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P2Beta1BlockBlockType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BARCODE" => GoogleCloudVisionV1P2Beta1BlockBlockType::Barcode,
                "PICTURE" => GoogleCloudVisionV1P2Beta1BlockBlockType::Picture,
                "RULER" => GoogleCloudVisionV1P2Beta1BlockBlockType::Ruler,
                "TABLE" => GoogleCloudVisionV1P2Beta1BlockBlockType::Table,
                "TEXT" => GoogleCloudVisionV1P2Beta1BlockBlockType::Text,
                "UNKNOWN" => GoogleCloudVisionV1P2Beta1BlockBlockType::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1BlockBlockType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1BoundingPoly {
        #[doc = "The bounding polygon normalized vertices."]
        #[serde(rename = "normalizedVertices", default)]
        pub normalized_vertices:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P2Beta1NormalizedVertex>>,
        #[doc = "The bounding polygon vertices."]
        #[serde(rename = "vertices", default)]
        pub vertices: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P2Beta1Vertex>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1BoundingPoly {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1ColorInfo {
        #[doc = "RGB components of the color."]
        #[serde(rename = "color", default)]
        pub color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The fraction of pixels the color occupies in the image.\nValue in range [0, 1]."]
        #[serde(rename = "pixelFraction", default)]
        pub pixel_fraction: ::std::option::Option<f32>,
        #[doc = "Image-specific score for this color. Value in range [0, 1]."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1ColorInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1CropHint {
        #[doc = "The bounding polygon for the crop region. The coordinates of the bounding\nbox are in the original image's scale."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P2Beta1BoundingPoly>,
        #[doc = "Confidence of this being a salient region.  Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Fraction of importance of this salient region with respect to the original\nimage."]
        #[serde(rename = "importanceFraction", default)]
        pub importance_fraction: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1CropHint {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1CropHintsAnnotation {
        #[doc = "Crop hint results."]
        #[serde(rename = "cropHints", default)]
        pub crop_hints:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P2Beta1CropHint>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1CropHintsAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1DominantColorsAnnotation {
        #[doc = "RGB color values with their score and pixel fraction."]
        #[serde(rename = "colors", default)]
        pub colors: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P2Beta1ColorInfo>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1DominantColorsAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1EntityAnnotation {
        #[doc = "Image region to which this entity belongs. Not produced\nfor `LABEL_DETECTION` features."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P2Beta1BoundingPoly>,
        #[doc = "**Deprecated. Use `score` instead.**\nThe accuracy of the entity detection in an image.\nFor example, for an image in which the \"Eiffel Tower\" entity is detected,\nthis field represents the confidence that there is a tower in the query\nimage. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Entity textual description, expressed in its `locale` language."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "The language code for the locale in which the entity textual\n`description` is expressed."]
        #[serde(rename = "locale", default)]
        pub locale: ::std::option::Option<String>,
        #[doc = "The location information for the detected entity. Multiple\n`LocationInfo` elements can be present because one location may\nindicate the location of the scene in the image, and another location\nmay indicate the location of the place where the image was taken.\nLocation information is usually present for landmarks."]
        #[serde(rename = "locations", default)]
        pub locations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P2Beta1LocationInfo>>,
        #[doc = "Opaque entity ID. Some IDs may be available in\n[Google Knowledge Graph Search\nAPI](https://developers.google.com/knowledge-graph/)."]
        #[serde(rename = "mid", default)]
        pub mid: ::std::option::Option<String>,
        #[doc = "Some entities may have optional user-supplied `Property` (name/value)\nfields, such a score or string that qualifies the entity."]
        #[serde(rename = "properties", default)]
        pub properties:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P2Beta1Property>>,
        #[doc = "Overall score of the result. Range [0, 1]."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
        #[doc = "The relevancy of the ICA (Image Content Annotation) label to the\nimage. For example, the relevancy of \"tower\" is likely higher to an image\ncontaining the detected \"Eiffel Tower\" than to an image containing a\ndetected distant towering building, even though the confidence that\nthere is a tower in each image may be the same. Range [0, 1]."]
        #[serde(rename = "topicality", default)]
        pub topicality: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1EntityAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1FaceAnnotation {
        #[doc = "Anger likelihood."]
        #[serde(rename = "angerLikelihood", default)]
        pub anger_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P2Beta1FaceAnnotationAngerLikelihood,
        >,
        #[doc = "Blurred likelihood."]
        #[serde(rename = "blurredLikelihood", default)]
        pub blurred_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P2Beta1FaceAnnotationBlurredLikelihood,
        >,
        #[doc = "The bounding polygon around the face. The coordinates of the bounding box\nare in the original image's scale.\nThe bounding box is computed to \"frame\" the face in accordance with human\nexpectations. It is based on the landmarker results.\nNote that one or more x and/or y coordinates may not be generated in the\n`BoundingPoly` (the polygon will be unbounded) if only a partial face\nappears in the image to be annotated."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P2Beta1BoundingPoly>,
        #[doc = "Detection confidence. Range [0, 1]."]
        #[serde(rename = "detectionConfidence", default)]
        pub detection_confidence: ::std::option::Option<f32>,
        #[doc = "The `fd_bounding_poly` bounding polygon is tighter than the\n`boundingPoly`, and encloses only the skin part of the face. Typically, it\nis used to eliminate the face from any image analysis that detects the\n\"amount of skin\" visible in an image. It is not based on the\nlandmarker results, only on the initial face detection, hence\nthe <code>fd</code> (face detection) prefix."]
        #[serde(rename = "fdBoundingPoly", default)]
        pub fd_bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P2Beta1BoundingPoly>,
        #[doc = "Headwear likelihood."]
        #[serde(rename = "headwearLikelihood", default)]
        pub headwear_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P2Beta1FaceAnnotationHeadwearLikelihood,
        >,
        #[doc = "Joy likelihood."]
        #[serde(rename = "joyLikelihood", default)]
        pub joy_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P2Beta1FaceAnnotationJoyLikelihood,
        >,
        #[doc = "Face landmarking confidence. Range [0, 1]."]
        #[serde(rename = "landmarkingConfidence", default)]
        pub landmarking_confidence: ::std::option::Option<f32>,
        #[doc = "Detected face landmarks."]
        #[serde(rename = "landmarks", default)]
        pub landmarks: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P2Beta1FaceAnnotationLandmark>,
        >,
        #[doc = "Yaw angle, which indicates the leftward/rightward angle that the face is\npointing relative to the vertical plane perpendicular to the image. Range\n[-180,180]."]
        #[serde(rename = "panAngle", default)]
        pub pan_angle: ::std::option::Option<f32>,
        #[doc = "Roll angle, which indicates the amount of clockwise/anti-clockwise rotation\nof the face relative to the image vertical about the axis perpendicular to\nthe face. Range [-180,180]."]
        #[serde(rename = "rollAngle", default)]
        pub roll_angle: ::std::option::Option<f32>,
        #[doc = "Sorrow likelihood."]
        #[serde(rename = "sorrowLikelihood", default)]
        pub sorrow_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P2Beta1FaceAnnotationSorrowLikelihood,
        >,
        #[doc = "Surprise likelihood."]
        #[serde(rename = "surpriseLikelihood", default)]
        pub surprise_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P2Beta1FaceAnnotationSurpriseLikelihood,
        >,
        #[doc = "Pitch angle, which indicates the upwards/downwards angle that the face is\npointing relative to the image's horizontal plane. Range [-180,180]."]
        #[serde(rename = "tiltAngle", default)]
        pub tilt_angle: ::std::option::Option<f32>,
        #[doc = "Under-exposed likelihood."]
        #[serde(rename = "underExposedLikelihood", default)]
        pub under_exposed_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P2Beta1FaceAnnotationUnderExposedLikelihood,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1FaceAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P2Beta1FaceAnnotationAngerLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P2Beta1FaceAnnotationAngerLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P2Beta1FaceAnnotationAngerLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P2Beta1FaceAnnotationAngerLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P2Beta1FaceAnnotationAngerLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P2Beta1FaceAnnotationAngerLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P2Beta1FaceAnnotationAngerLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationAngerLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P2Beta1FaceAnnotationAngerLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P2Beta1FaceAnnotationAngerLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P2Beta1FaceAnnotationAngerLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P2Beta1FaceAnnotationAngerLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P2Beta1FaceAnnotationAngerLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P2Beta1FaceAnnotationAngerLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P2Beta1FaceAnnotationAngerLikelihood::Unlikely,
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationAngerLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationAngerLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1FaceAnnotationAngerLikelihood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P2Beta1FaceAnnotationBlurredLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P2Beta1FaceAnnotationBlurredLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P2Beta1FaceAnnotationBlurredLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P2Beta1FaceAnnotationBlurredLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P2Beta1FaceAnnotationBlurredLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P2Beta1FaceAnnotationBlurredLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P2Beta1FaceAnnotationBlurredLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationBlurredLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P2Beta1FaceAnnotationBlurredLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P2Beta1FaceAnnotationBlurredLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P2Beta1FaceAnnotationBlurredLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P2Beta1FaceAnnotationBlurredLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P2Beta1FaceAnnotationBlurredLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P2Beta1FaceAnnotationBlurredLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P2Beta1FaceAnnotationBlurredLikelihood::Unlikely,
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationBlurredLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationBlurredLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1FaceAnnotationBlurredLikelihood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P2Beta1FaceAnnotationHeadwearLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P2Beta1FaceAnnotationHeadwearLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P2Beta1FaceAnnotationHeadwearLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P2Beta1FaceAnnotationHeadwearLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P2Beta1FaceAnnotationHeadwearLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P2Beta1FaceAnnotationHeadwearLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P2Beta1FaceAnnotationHeadwearLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationHeadwearLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P2Beta1FaceAnnotationHeadwearLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P2Beta1FaceAnnotationHeadwearLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P2Beta1FaceAnnotationHeadwearLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P2Beta1FaceAnnotationHeadwearLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P2Beta1FaceAnnotationHeadwearLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P2Beta1FaceAnnotationHeadwearLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P2Beta1FaceAnnotationHeadwearLikelihood::Unlikely,
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationHeadwearLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationHeadwearLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVisionV1P2Beta1FaceAnnotationHeadwearLikelihood
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P2Beta1FaceAnnotationJoyLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P2Beta1FaceAnnotationJoyLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P2Beta1FaceAnnotationJoyLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P2Beta1FaceAnnotationJoyLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P2Beta1FaceAnnotationJoyLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P2Beta1FaceAnnotationJoyLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P2Beta1FaceAnnotationJoyLikelihood::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P2Beta1FaceAnnotationJoyLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P2Beta1FaceAnnotationJoyLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P2Beta1FaceAnnotationJoyLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P2Beta1FaceAnnotationJoyLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P2Beta1FaceAnnotationJoyLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P2Beta1FaceAnnotationJoyLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P2Beta1FaceAnnotationJoyLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P2Beta1FaceAnnotationJoyLikelihood::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P2Beta1FaceAnnotationJoyLikelihood::VeryLikely,
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationJoyLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1FaceAnnotationJoyLikelihood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P2Beta1FaceAnnotationSorrowLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P2Beta1FaceAnnotationSorrowLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P2Beta1FaceAnnotationSorrowLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P2Beta1FaceAnnotationSorrowLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P2Beta1FaceAnnotationSorrowLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P2Beta1FaceAnnotationSorrowLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P2Beta1FaceAnnotationSorrowLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationSorrowLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P2Beta1FaceAnnotationSorrowLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P2Beta1FaceAnnotationSorrowLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P2Beta1FaceAnnotationSorrowLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P2Beta1FaceAnnotationSorrowLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P2Beta1FaceAnnotationSorrowLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P2Beta1FaceAnnotationSorrowLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P2Beta1FaceAnnotationSorrowLikelihood::Unlikely,
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationSorrowLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationSorrowLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1FaceAnnotationSorrowLikelihood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P2Beta1FaceAnnotationSurpriseLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P2Beta1FaceAnnotationSurpriseLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P2Beta1FaceAnnotationSurpriseLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P2Beta1FaceAnnotationSurpriseLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P2Beta1FaceAnnotationSurpriseLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P2Beta1FaceAnnotationSurpriseLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P2Beta1FaceAnnotationSurpriseLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationSurpriseLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P2Beta1FaceAnnotationSurpriseLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P2Beta1FaceAnnotationSurpriseLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P2Beta1FaceAnnotationSurpriseLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P2Beta1FaceAnnotationSurpriseLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P2Beta1FaceAnnotationSurpriseLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P2Beta1FaceAnnotationSurpriseLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P2Beta1FaceAnnotationSurpriseLikelihood::Unlikely,
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationSurpriseLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationSurpriseLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVisionV1P2Beta1FaceAnnotationSurpriseLikelihood
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P2Beta1FaceAnnotationUnderExposedLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P2Beta1FaceAnnotationUnderExposedLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P2Beta1FaceAnnotationUnderExposedLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P2Beta1FaceAnnotationUnderExposedLikelihood::Possible => {
                    "POSSIBLE"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationUnderExposedLikelihood::Unknown => {
                    "UNKNOWN"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationUnderExposedLikelihood::Unlikely => {
                    "UNLIKELY"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationUnderExposedLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationUnderExposedLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P2Beta1FaceAnnotationUnderExposedLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P2Beta1FaceAnnotationUnderExposedLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVisionV1P2Beta1FaceAnnotationUnderExposedLikelihood
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P2Beta1FaceAnnotationUnderExposedLikelihood::Likely,
                "POSSIBLE" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationUnderExposedLikelihood::Possible
                }
                "UNKNOWN" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationUnderExposedLikelihood::Unknown
                }
                "UNLIKELY" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationUnderExposedLikelihood::Unlikely
                }
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationUnderExposedLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationUnderExposedLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVisionV1P2Beta1FaceAnnotationUnderExposedLikelihood
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1FaceAnnotationLandmark {
        #[doc = "Face landmark position."]
        #[serde(rename = "position", default)]
        pub position: ::std::option::Option<crate::schemas::GoogleCloudVisionV1P2Beta1Position>,
        #[doc = "Face landmark type."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1FaceAnnotationLandmark {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType {
        #[doc = "Chin gnathion."]
        ChinGnathion,
        #[doc = "Chin left gonion."]
        ChinLeftGonion,
        #[doc = "Chin right gonion."]
        ChinRightGonion,
        #[doc = "Forehead glabella."]
        ForeheadGlabella,
        #[doc = "Left ear tragion."]
        LeftEarTragion,
        #[doc = "Left eye."]
        LeftEye,
        #[doc = "Left eye, bottom boundary."]
        LeftEyeBottomBoundary,
        #[doc = "Left eye, left corner."]
        LeftEyeLeftCorner,
        #[doc = "Left eye pupil."]
        LeftEyePupil,
        #[doc = "Left eye, right corner."]
        LeftEyeRightCorner,
        #[doc = "Left eye, top boundary."]
        LeftEyeTopBoundary,
        #[doc = "Left eyebrow, upper midpoint."]
        LeftEyebrowUpperMidpoint,
        #[doc = "Left of left eyebrow."]
        LeftOfLeftEyebrow,
        #[doc = "Left of right eyebrow."]
        LeftOfRightEyebrow,
        #[doc = "Lower lip."]
        LowerLip,
        #[doc = "Midpoint between eyes."]
        MidpointBetweenEyes,
        #[doc = "Mouth center."]
        MouthCenter,
        #[doc = "Mouth left."]
        MouthLeft,
        #[doc = "Mouth right."]
        MouthRight,
        #[doc = "Nose, bottom center."]
        NoseBottomCenter,
        #[doc = "Nose, bottom left."]
        NoseBottomLeft,
        #[doc = "Nose, bottom right."]
        NoseBottomRight,
        #[doc = "Nose tip."]
        NoseTip,
        #[doc = "Right ear tragion."]
        RightEarTragion,
        #[doc = "Right eye."]
        RightEye,
        #[doc = "Right eye, bottom boundary."]
        RightEyeBottomBoundary,
        #[doc = "Right eye, left corner."]
        RightEyeLeftCorner,
        #[doc = "Right eye pupil."]
        RightEyePupil,
        #[doc = "Right eye, right corner."]
        RightEyeRightCorner,
        #[doc = "Right eye, top boundary."]
        RightEyeTopBoundary,
        #[doc = "Right eyebrow, upper midpoint."]
        RightEyebrowUpperMidpoint,
        #[doc = "Right of left eyebrow."]
        RightOfLeftEyebrow,
        #[doc = "Right of right eyebrow."]
        RightOfRightEyebrow,
        #[doc = "Unknown face landmark detected. Should not be filled."]
        UnknownLandmark,
        #[doc = "Upper lip."]
        UpperLip,
    }
    impl GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::ChinGnathion => {
                    "CHIN_GNATHION"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::ChinLeftGonion => {
                    "CHIN_LEFT_GONION"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::ChinRightGonion => {
                    "CHIN_RIGHT_GONION"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::ForeheadGlabella => {
                    "FOREHEAD_GLABELLA"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::LeftEarTragion => {
                    "LEFT_EAR_TRAGION"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::LeftEye => "LEFT_EYE",
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::LeftEyeBottomBoundary => {
                    "LEFT_EYE_BOTTOM_BOUNDARY"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::LeftEyeLeftCorner => {
                    "LEFT_EYE_LEFT_CORNER"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::LeftEyePupil => {
                    "LEFT_EYE_PUPIL"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::LeftEyeRightCorner => {
                    "LEFT_EYE_RIGHT_CORNER"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::LeftEyeTopBoundary => {
                    "LEFT_EYE_TOP_BOUNDARY"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::LeftEyebrowUpperMidpoint => {
                    "LEFT_EYEBROW_UPPER_MIDPOINT"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::LeftOfLeftEyebrow => {
                    "LEFT_OF_LEFT_EYEBROW"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::LeftOfRightEyebrow => {
                    "LEFT_OF_RIGHT_EYEBROW"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::LowerLip => "LOWER_LIP",
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::MidpointBetweenEyes => {
                    "MIDPOINT_BETWEEN_EYES"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::MouthCenter => "MOUTH_CENTER",
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::MouthLeft => "MOUTH_LEFT",
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::MouthRight => "MOUTH_RIGHT",
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::NoseBottomCenter => {
                    "NOSE_BOTTOM_CENTER"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::NoseBottomLeft => {
                    "NOSE_BOTTOM_LEFT"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::NoseBottomRight => {
                    "NOSE_BOTTOM_RIGHT"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::NoseTip => "NOSE_TIP",
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::RightEarTragion => {
                    "RIGHT_EAR_TRAGION"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::RightEye => "RIGHT_EYE",
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::RightEyeBottomBoundary => {
                    "RIGHT_EYE_BOTTOM_BOUNDARY"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::RightEyeLeftCorner => {
                    "RIGHT_EYE_LEFT_CORNER"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::RightEyePupil => {
                    "RIGHT_EYE_PUPIL"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::RightEyeRightCorner => {
                    "RIGHT_EYE_RIGHT_CORNER"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::RightEyeTopBoundary => {
                    "RIGHT_EYE_TOP_BOUNDARY"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::RightEyebrowUpperMidpoint => {
                    "RIGHT_EYEBROW_UPPER_MIDPOINT"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::RightOfLeftEyebrow => {
                    "RIGHT_OF_LEFT_EYEBROW"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::RightOfRightEyebrow => {
                    "RIGHT_OF_RIGHT_EYEBROW"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::UnknownLandmark => {
                    "UNKNOWN_LANDMARK"
                }
                GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::UpperLip => "UPPER_LIP",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CHIN_GNATHION" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::ChinGnathion
                }
                "CHIN_LEFT_GONION" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::ChinLeftGonion
                }
                "CHIN_RIGHT_GONION" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::ChinRightGonion
                }
                "FOREHEAD_GLABELLA" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::ForeheadGlabella
                }
                "LEFT_EAR_TRAGION" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::LeftEarTragion
                }
                "LEFT_EYE" => GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::LeftEye,
                "LEFT_EYE_BOTTOM_BOUNDARY" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::LeftEyeBottomBoundary
                }
                "LEFT_EYE_LEFT_CORNER" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::LeftEyeLeftCorner
                }
                "LEFT_EYE_PUPIL" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::LeftEyePupil
                }
                "LEFT_EYE_RIGHT_CORNER" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::LeftEyeRightCorner
                }
                "LEFT_EYE_TOP_BOUNDARY" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::LeftEyeTopBoundary
                }
                "LEFT_EYEBROW_UPPER_MIDPOINT" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::LeftEyebrowUpperMidpoint
                }
                "LEFT_OF_LEFT_EYEBROW" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::LeftOfLeftEyebrow
                }
                "LEFT_OF_RIGHT_EYEBROW" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::LeftOfRightEyebrow
                }
                "LOWER_LIP" => GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::LowerLip,
                "MIDPOINT_BETWEEN_EYES" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::MidpointBetweenEyes
                }
                "MOUTH_CENTER" => GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::MouthCenter,
                "MOUTH_LEFT" => GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::MouthLeft,
                "MOUTH_RIGHT" => GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::MouthRight,
                "NOSE_BOTTOM_CENTER" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::NoseBottomCenter
                }
                "NOSE_BOTTOM_LEFT" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::NoseBottomLeft
                }
                "NOSE_BOTTOM_RIGHT" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::NoseBottomRight
                }
                "NOSE_TIP" => GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::NoseTip,
                "RIGHT_EAR_TRAGION" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::RightEarTragion
                }
                "RIGHT_EYE" => GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::RightEye,
                "RIGHT_EYE_BOTTOM_BOUNDARY" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::RightEyeBottomBoundary
                }
                "RIGHT_EYE_LEFT_CORNER" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::RightEyeLeftCorner
                }
                "RIGHT_EYE_PUPIL" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::RightEyePupil
                }
                "RIGHT_EYE_RIGHT_CORNER" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::RightEyeRightCorner
                }
                "RIGHT_EYE_TOP_BOUNDARY" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::RightEyeTopBoundary
                }
                "RIGHT_EYEBROW_UPPER_MIDPOINT" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::RightEyebrowUpperMidpoint
                }
                "RIGHT_OF_LEFT_EYEBROW" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::RightOfLeftEyebrow
                }
                "RIGHT_OF_RIGHT_EYEBROW" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::RightOfRightEyebrow
                }
                "UNKNOWN_LANDMARK" => {
                    GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::UnknownLandmark
                }
                "UPPER_LIP" => GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType::UpperLip,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1FaceAnnotationLandmarkType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1GcsDestination {
        #[doc = "Google Cloud Storage URI prefix where the results will be stored. Results\nwill be in JSON format and preceded by its corresponding input URI prefix.\nThis field can either represent a gcs file prefix or gcs directory. In\neither case, the uri should be unique because in order to get all of the\noutput files, you will need to do a wildcard gcs search on the uri prefix\nyou provide.\n\nExamples:\n\n* File Prefix: gs://bucket-name/here/filenameprefix   The output files\n  will be created in gs://bucket-name/here/ and the names of the\n  output files will begin with \"filenameprefix\".\n\n* Directory Prefix: gs://bucket-name/some/location/   The output files\n  will be created in gs://bucket-name/some/location/ and the names of the\n  output files could be anything because there was no filename prefix\n  specified.\n\nIf multiple outputs, each response is still AnnotateFileResponse, each of\nwhich contains some subset of the full list of AnnotateImageResponse.\nMultiple outputs can happen if, for example, the output JSON is too large\nand overflows into multiple sharded files."]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1GcsDestination {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1GcsSource {
        #[doc = "Google Cloud Storage URI for the input file. This must only be a\nGoogle Cloud Storage object. Wildcards are not currently supported."]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1GcsSource {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1ImageAnnotationContext {
        #[doc = "If the file was a PDF or TIFF, this field gives the page number within\nthe file used to produce the image."]
        #[serde(rename = "pageNumber", default)]
        pub page_number: ::std::option::Option<i32>,
        #[doc = "The URI of the file used to produce the image."]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1ImageAnnotationContext {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1ImageProperties {
        #[doc = "If present, dominant colors completed successfully."]
        #[serde(rename = "dominantColors", default)]
        pub dominant_colors: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P2Beta1DominantColorsAnnotation,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1ImageProperties {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1InputConfig {
        #[doc = "File content, represented as a stream of bytes.\nNote: As with all `bytes` fields, protobuffers use a pure binary\nrepresentation, whereas JSON representations use base64.\n\nCurrently, this field only works for BatchAnnotateFiles requests. It does\nnot work for AsyncBatchAnnotateFiles requests."]
        #[serde(rename = "content", default)]
        pub content: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "The Google Cloud Storage location to read the input from."]
        #[serde(rename = "gcsSource", default)]
        pub gcs_source: ::std::option::Option<crate::schemas::GoogleCloudVisionV1P2Beta1GcsSource>,
        #[doc = "The type of the file. Currently only \"application/pdf\", \"image/tiff\" and\n\"image/gif\" are supported. Wildcards are not supported."]
        #[serde(rename = "mimeType", default)]
        pub mime_type: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1InputConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1LocalizedObjectAnnotation {
        #[doc = "Image region to which this object belongs. This must be populated."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P2Beta1BoundingPoly>,
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more\ninformation, see\nhttp://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Object ID that should align with EntityAnnotation mid."]
        #[serde(rename = "mid", default)]
        pub mid: ::std::option::Option<String>,
        #[doc = "Object name, expressed in its `language_code` language."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Score of the result. Range [0, 1]."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1LocalizedObjectAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1LocationInfo {
        #[doc = "lat/long location coordinates."]
        #[serde(rename = "latLng", default)]
        pub lat_lng: ::std::option::Option<crate::schemas::LatLng>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1LocationInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1NormalizedVertex {
        #[doc = "X coordinate."]
        #[serde(rename = "x", default)]
        pub x: ::std::option::Option<f32>,
        #[doc = "Y coordinate."]
        #[serde(rename = "y", default)]
        pub y: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1NormalizedVertex {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1OperationMetadata {
        #[doc = "The time when the batch request was received."]
        #[serde(rename = "createTime", default)]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Current state of the batch operation."]
        #[serde(rename = "state", default)]
        pub state:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P2Beta1OperationMetadataState>,
        #[doc = "The time when the operation result was last updated."]
        #[serde(rename = "updateTime", default)]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1OperationMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P2Beta1OperationMetadataState {
        #[doc = "The batch processing was cancelled."]
        Cancelled,
        #[doc = "Request is received."]
        Created,
        #[doc = "The batch processing is done."]
        Done,
        #[doc = "Request is actively being processed."]
        Running,
        #[doc = "Invalid."]
        StateUnspecified,
    }
    impl GoogleCloudVisionV1P2Beta1OperationMetadataState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P2Beta1OperationMetadataState::Cancelled => "CANCELLED",
                GoogleCloudVisionV1P2Beta1OperationMetadataState::Created => "CREATED",
                GoogleCloudVisionV1P2Beta1OperationMetadataState::Done => "DONE",
                GoogleCloudVisionV1P2Beta1OperationMetadataState::Running => "RUNNING",
                GoogleCloudVisionV1P2Beta1OperationMetadataState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P2Beta1OperationMetadataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P2Beta1OperationMetadataState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P2Beta1OperationMetadataState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => GoogleCloudVisionV1P2Beta1OperationMetadataState::Cancelled,
                "CREATED" => GoogleCloudVisionV1P2Beta1OperationMetadataState::Created,
                "DONE" => GoogleCloudVisionV1P2Beta1OperationMetadataState::Done,
                "RUNNING" => GoogleCloudVisionV1P2Beta1OperationMetadataState::Running,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudVisionV1P2Beta1OperationMetadataState::StateUnspecified
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1OperationMetadataState {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1OutputConfig {
        #[doc = "The max number of response protos to put into each output JSON file on\nGoogle Cloud Storage.\nThe valid range is [1, 100]. If not specified, the default value is 20.\n\nFor example, for one pdf file with 100 pages, 100 response protos will\nbe generated. If `batch_size` = 20, then 5 json files each\ncontaining 20 response protos will be written under the prefix\n`gcs_destination`.`uri`.\n\nCurrently, batch_size only applies to GcsDestination, with potential future\nsupport for other output configurations."]
        #[serde(rename = "batchSize", default)]
        pub batch_size: ::std::option::Option<i32>,
        #[doc = "The Google Cloud Storage location to write the output(s) to."]
        #[serde(rename = "gcsDestination", default)]
        pub gcs_destination:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P2Beta1GcsDestination>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1OutputConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1Page {
        #[doc = "List of blocks of text, images etc on this page."]
        #[serde(rename = "blocks", default)]
        pub blocks: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P2Beta1Block>>,
        #[doc = "Confidence of the OCR results on the page. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Page height. For PDFs the unit is points. For images (including\nTIFFs) the unit is pixels."]
        #[serde(rename = "height", default)]
        pub height: ::std::option::Option<i32>,
        #[doc = "Additional information detected on the page."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P2Beta1TextAnnotationTextProperty,
        >,
        #[doc = "Page width. For PDFs the unit is points. For images (including\nTIFFs) the unit is pixels."]
        #[serde(rename = "width", default)]
        pub width: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1Page {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1Paragraph {
        #[doc = "The bounding box for the paragraph.\nThe vertices are in the order of top-left, top-right, bottom-right,\nbottom-left. When a rotation of the bounding box is detected the rotation\nis represented as around the top-left corner as defined when the text is\nread in the 'natural' orientation.\nFor example:\n\n* when the text is horizontal it might look like:\n  0----1\n  |    |\n  3----2\n* when it's rotated 180 degrees around the top-left corner it becomes:\n  2----3\n  |    |\n  1----0\n  and the vertex order will still be (0, 1, 2, 3)."]
        #[serde(rename = "boundingBox", default)]
        pub bounding_box:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P2Beta1BoundingPoly>,
        #[doc = "Confidence of the OCR results for the paragraph. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Additional information detected for the paragraph."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P2Beta1TextAnnotationTextProperty,
        >,
        #[doc = "List of all words in this paragraph."]
        #[serde(rename = "words", default)]
        pub words: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P2Beta1Word>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1Paragraph {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1Position {
        #[doc = "X coordinate."]
        #[serde(rename = "x", default)]
        pub x: ::std::option::Option<f32>,
        #[doc = "Y coordinate."]
        #[serde(rename = "y", default)]
        pub y: ::std::option::Option<f32>,
        #[doc = "Z coordinate (or depth)."]
        #[serde(rename = "z", default)]
        pub z: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1Position {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1Product {
        #[doc = "User-provided metadata to be stored with this product. Must be at most 4096\ncharacters long."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "The user-provided name for this Product. Must not be empty. Must be at most\n4096 characters long."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The resource name of the product.\n\nFormat is:\n`projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`.\n\nThis field is ignored when creating a product."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The category for the product identified by the reference image. This should\nbe either \"homegoods-v2\", \"apparel-v2\", or \"toys-v2\". The legacy categories\n\"homegoods\", \"apparel\", and \"toys\" are still supported, but these should\nnot be used for new products.\n\nThis field is immutable."]
        #[serde(rename = "productCategory", default)]
        pub product_category: ::std::option::Option<String>,
        #[doc = "Key-value pairs that can be attached to a product. At query time,\nconstraints can be specified based on the product_labels.\n\nNote that integer values can be provided as strings, e.g. \"1199\". Only\nstrings with integer values can match a range-based restriction which is\nto be supported soon.\n\nMultiple values can be assigned to the same key. One product may have up to\n500 product_labels.\n\nNotice that the total number of distinct product_labels over all products\nin one ProductSet cannot exceed 1M, otherwise the product search pipeline\nwill refuse to work for that ProductSet."]
        #[serde(rename = "productLabels", default)]
        pub product_labels:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P2Beta1ProductKeyValue>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1Product {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1ProductKeyValue {
        #[doc = "The key of the label attached to the product. Cannot be empty and cannot\nexceed 128 bytes."]
        #[serde(rename = "key", default)]
        pub key: ::std::option::Option<String>,
        #[doc = "The value of the label attached to the product. Cannot be empty and\ncannot exceed 128 bytes."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1ProductKeyValue {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1ProductSearchResults {
        #[doc = "Timestamp of the index which provided these results. Products added to the\nproduct set and products removed from the product set after this time are\nnot reflected in the current results."]
        #[serde(rename = "indexTime", default)]
        pub index_time: ::std::option::Option<String>,
        #[doc = "List of results grouped by products detected in the query image. Each entry\ncorresponds to one bounding polygon in the query image, and contains the\nmatching products specific to that region. There may be duplicate product\nmatches in the union of all the per-product results."]
        #[serde(rename = "productGroupedResults", default)]
        pub product_grouped_results: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P2Beta1ProductSearchResultsGroupedResult>,
        >,
        #[doc = "List of results, one for each product match."]
        #[serde(rename = "results", default)]
        pub results: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P2Beta1ProductSearchResultsResult>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1ProductSearchResults {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1ProductSearchResultsGroupedResult {
        #[doc = "The bounding polygon around the product detected in the query image."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P2Beta1BoundingPoly>,
        #[doc = "List of results, one for each product match."]
        #[serde(rename = "results", default)]
        pub results: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P2Beta1ProductSearchResultsResult>,
        >,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVisionV1P2Beta1ProductSearchResultsGroupedResult
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1ProductSearchResultsResult {
        #[doc = "The resource name of the image from the product that is the closest match\nto the query."]
        #[serde(rename = "image", default)]
        pub image: ::std::option::Option<String>,
        #[doc = "The Product."]
        #[serde(rename = "product", default)]
        pub product: ::std::option::Option<crate::schemas::GoogleCloudVisionV1P2Beta1Product>,
        #[doc = "A confidence level on the match, ranging from 0 (no confidence) to\n1 (full confidence)."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1ProductSearchResultsResult {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1Property {
        #[doc = "Name of the property."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Value of numeric properties."]
        #[serde(rename = "uint64Value", default)]
        #[serde(with = "crate::parsed_string")]
        pub uint_64_value: ::std::option::Option<u64>,
        #[doc = "Value of the property."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1Property {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1SafeSearchAnnotation {
        #[doc = "Represents the adult content likelihood for the image. Adult content may\ncontain elements such as nudity, pornographic images or cartoons, or\nsexual activities."]
        #[serde(rename = "adult", default)]
        pub adult: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P2Beta1SafeSearchAnnotationAdult,
        >,
        #[doc = "Likelihood that this is a medical image."]
        #[serde(rename = "medical", default)]
        pub medical: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P2Beta1SafeSearchAnnotationMedical,
        >,
        #[doc = "Likelihood that the request image contains racy content. Racy content may\ninclude (but is not limited to) skimpy or sheer clothing, strategically\ncovered nudity, lewd or provocative poses, or close-ups of sensitive\nbody areas."]
        #[serde(rename = "racy", default)]
        pub racy: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P2Beta1SafeSearchAnnotationRacy,
        >,
        #[doc = "Spoof likelihood. The likelihood that an modification\nwas made to the image's canonical version to make it appear\nfunny or offensive."]
        #[serde(rename = "spoof", default)]
        pub spoof: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P2Beta1SafeSearchAnnotationSpoof,
        >,
        #[doc = "Likelihood that this image contains violent content."]
        #[serde(rename = "violence", default)]
        pub violence: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P2Beta1SafeSearchAnnotationViolence,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1SafeSearchAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P2Beta1SafeSearchAnnotationAdult {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P2Beta1SafeSearchAnnotationAdult {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationAdult::Likely => "LIKELY",
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationAdult::Possible => "POSSIBLE",
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationAdult::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationAdult::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationAdult::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationAdult::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P2Beta1SafeSearchAnnotationAdult {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P2Beta1SafeSearchAnnotationAdult {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P2Beta1SafeSearchAnnotationAdult {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P2Beta1SafeSearchAnnotationAdult::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P2Beta1SafeSearchAnnotationAdult::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P2Beta1SafeSearchAnnotationAdult::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P2Beta1SafeSearchAnnotationAdult::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P2Beta1SafeSearchAnnotationAdult::VeryLikely,
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P2Beta1SafeSearchAnnotationAdult::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1SafeSearchAnnotationAdult {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P2Beta1SafeSearchAnnotationMedical {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P2Beta1SafeSearchAnnotationMedical {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationMedical::Likely => "LIKELY",
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationMedical::Possible => "POSSIBLE",
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationMedical::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationMedical::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationMedical::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationMedical::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P2Beta1SafeSearchAnnotationMedical {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P2Beta1SafeSearchAnnotationMedical {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P2Beta1SafeSearchAnnotationMedical {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P2Beta1SafeSearchAnnotationMedical::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P2Beta1SafeSearchAnnotationMedical::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P2Beta1SafeSearchAnnotationMedical::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P2Beta1SafeSearchAnnotationMedical::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P2Beta1SafeSearchAnnotationMedical::VeryLikely,
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P2Beta1SafeSearchAnnotationMedical::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1SafeSearchAnnotationMedical {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P2Beta1SafeSearchAnnotationRacy {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P2Beta1SafeSearchAnnotationRacy {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationRacy::Likely => "LIKELY",
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationRacy::Possible => "POSSIBLE",
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationRacy::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationRacy::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationRacy::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationRacy::VeryUnlikely => "VERY_UNLIKELY",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P2Beta1SafeSearchAnnotationRacy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P2Beta1SafeSearchAnnotationRacy {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P2Beta1SafeSearchAnnotationRacy {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P2Beta1SafeSearchAnnotationRacy::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P2Beta1SafeSearchAnnotationRacy::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P2Beta1SafeSearchAnnotationRacy::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P2Beta1SafeSearchAnnotationRacy::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P2Beta1SafeSearchAnnotationRacy::VeryLikely,
                "VERY_UNLIKELY" => GoogleCloudVisionV1P2Beta1SafeSearchAnnotationRacy::VeryUnlikely,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1SafeSearchAnnotationRacy {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P2Beta1SafeSearchAnnotationSpoof {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P2Beta1SafeSearchAnnotationSpoof {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationSpoof::Likely => "LIKELY",
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationSpoof::Possible => "POSSIBLE",
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationSpoof::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationSpoof::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationSpoof::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationSpoof::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P2Beta1SafeSearchAnnotationSpoof {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P2Beta1SafeSearchAnnotationSpoof {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P2Beta1SafeSearchAnnotationSpoof {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P2Beta1SafeSearchAnnotationSpoof::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P2Beta1SafeSearchAnnotationSpoof::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P2Beta1SafeSearchAnnotationSpoof::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P2Beta1SafeSearchAnnotationSpoof::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P2Beta1SafeSearchAnnotationSpoof::VeryLikely,
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P2Beta1SafeSearchAnnotationSpoof::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1SafeSearchAnnotationSpoof {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P2Beta1SafeSearchAnnotationViolence {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P2Beta1SafeSearchAnnotationViolence {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationViolence::Likely => "LIKELY",
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationViolence::Possible => "POSSIBLE",
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationViolence::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationViolence::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationViolence::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P2Beta1SafeSearchAnnotationViolence::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P2Beta1SafeSearchAnnotationViolence {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P2Beta1SafeSearchAnnotationViolence {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P2Beta1SafeSearchAnnotationViolence {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P2Beta1SafeSearchAnnotationViolence::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P2Beta1SafeSearchAnnotationViolence::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P2Beta1SafeSearchAnnotationViolence::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P2Beta1SafeSearchAnnotationViolence::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P2Beta1SafeSearchAnnotationViolence::VeryLikely,
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P2Beta1SafeSearchAnnotationViolence::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1SafeSearchAnnotationViolence {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1Symbol {
        #[doc = "The bounding box for the symbol.\nThe vertices are in the order of top-left, top-right, bottom-right,\nbottom-left. When a rotation of the bounding box is detected the rotation\nis represented as around the top-left corner as defined when the text is\nread in the 'natural' orientation.\nFor example:\n\n* when the text is horizontal it might look like:\n  0----1\n  |    |\n  3----2\n* when it's rotated 180 degrees around the top-left corner it becomes:\n  2----3\n  |    |\n  1----0\n  and the vertex order will still be (0, 1, 2, 3)."]
        #[serde(rename = "boundingBox", default)]
        pub bounding_box:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P2Beta1BoundingPoly>,
        #[doc = "Confidence of the OCR results for the symbol. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Additional information detected for the symbol."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P2Beta1TextAnnotationTextProperty,
        >,
        #[doc = "The actual UTF-8 representation of the symbol."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1Symbol {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1TextAnnotation {
        #[doc = "List of pages detected by OCR."]
        #[serde(rename = "pages", default)]
        pub pages: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P2Beta1Page>>,
        #[doc = "UTF-8 text detected on the pages."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1TextAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1TextAnnotationDetectedBreak {
        #[doc = "True if break prepends the element."]
        #[serde(rename = "isPrefix", default)]
        pub is_prefix: ::std::option::Option<bool>,
        #[doc = "Detected break type."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P2Beta1TextAnnotationDetectedBreakType,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1TextAnnotationDetectedBreak {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P2Beta1TextAnnotationDetectedBreakType {
        #[doc = "Line-wrapping break."]
        EolSureSpace,
        #[doc = "End-line hyphen that is not present in text; does not co-occur with\n`SPACE`, `LEADER_SPACE`, or `LINE_BREAK`."]
        Hyphen,
        #[doc = "Line break that ends a paragraph."]
        LineBreak,
        #[doc = "Regular space."]
        Space,
        #[doc = "Sure space (very wide)."]
        SureSpace,
        #[doc = "Unknown break label type."]
        Unknown,
    }
    impl GoogleCloudVisionV1P2Beta1TextAnnotationDetectedBreakType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P2Beta1TextAnnotationDetectedBreakType::EolSureSpace => {
                    "EOL_SURE_SPACE"
                }
                GoogleCloudVisionV1P2Beta1TextAnnotationDetectedBreakType::Hyphen => "HYPHEN",
                GoogleCloudVisionV1P2Beta1TextAnnotationDetectedBreakType::LineBreak => {
                    "LINE_BREAK"
                }
                GoogleCloudVisionV1P2Beta1TextAnnotationDetectedBreakType::Space => "SPACE",
                GoogleCloudVisionV1P2Beta1TextAnnotationDetectedBreakType::SureSpace => {
                    "SURE_SPACE"
                }
                GoogleCloudVisionV1P2Beta1TextAnnotationDetectedBreakType::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P2Beta1TextAnnotationDetectedBreakType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P2Beta1TextAnnotationDetectedBreakType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P2Beta1TextAnnotationDetectedBreakType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EOL_SURE_SPACE" => {
                    GoogleCloudVisionV1P2Beta1TextAnnotationDetectedBreakType::EolSureSpace
                }
                "HYPHEN" => GoogleCloudVisionV1P2Beta1TextAnnotationDetectedBreakType::Hyphen,
                "LINE_BREAK" => {
                    GoogleCloudVisionV1P2Beta1TextAnnotationDetectedBreakType::LineBreak
                }
                "SPACE" => GoogleCloudVisionV1P2Beta1TextAnnotationDetectedBreakType::Space,
                "SURE_SPACE" => {
                    GoogleCloudVisionV1P2Beta1TextAnnotationDetectedBreakType::SureSpace
                }
                "UNKNOWN" => GoogleCloudVisionV1P2Beta1TextAnnotationDetectedBreakType::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1TextAnnotationDetectedBreakType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1TextAnnotationDetectedLanguage {
        #[doc = "Confidence of detected language. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more\ninformation, see\nhttp://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1TextAnnotationDetectedLanguage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1TextAnnotationTextProperty {
        #[doc = "Detected start or end of a text segment."]
        #[serde(rename = "detectedBreak", default)]
        pub detected_break: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P2Beta1TextAnnotationDetectedBreak,
        >,
        #[doc = "A list of detected languages together with confidence."]
        #[serde(rename = "detectedLanguages", default)]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P2Beta1TextAnnotationDetectedLanguage>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1TextAnnotationTextProperty {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1Vertex {
        #[doc = "X coordinate."]
        #[serde(rename = "x", default)]
        pub x: ::std::option::Option<i32>,
        #[doc = "Y coordinate."]
        #[serde(rename = "y", default)]
        pub y: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1Vertex {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1WebDetection {
        #[doc = "The service's best guess as to the topic of the request image.\nInferred from similar images on the open web."]
        #[serde(rename = "bestGuessLabels", default)]
        pub best_guess_labels: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P2Beta1WebDetectionWebLabel>,
        >,
        #[doc = "Fully matching images from the Internet.\nCan include resized copies of the query image."]
        #[serde(rename = "fullMatchingImages", default)]
        pub full_matching_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P2Beta1WebDetectionWebImage>,
        >,
        #[doc = "Web pages containing the matching images from the Internet."]
        #[serde(rename = "pagesWithMatchingImages", default)]
        pub pages_with_matching_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P2Beta1WebDetectionWebPage>,
        >,
        #[doc = "Partial matching images from the Internet.\nThose images are similar enough to share some key-point features. For\nexample an original image will likely have partial matching for its crops."]
        #[serde(rename = "partialMatchingImages", default)]
        pub partial_matching_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P2Beta1WebDetectionWebImage>,
        >,
        #[doc = "The visually similar image results."]
        #[serde(rename = "visuallySimilarImages", default)]
        pub visually_similar_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P2Beta1WebDetectionWebImage>,
        >,
        #[doc = "Deduced entities from similar images on the Internet."]
        #[serde(rename = "webEntities", default)]
        pub web_entities: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P2Beta1WebDetectionWebEntity>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1WebDetection {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1WebDetectionWebEntity {
        #[doc = "Canonical description of the entity, in English."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Opaque entity ID."]
        #[serde(rename = "entityId", default)]
        pub entity_id: ::std::option::Option<String>,
        #[doc = "Overall relevancy score for the entity.\nNot normalized and not comparable across different image queries."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1WebDetectionWebEntity {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1WebDetectionWebImage {
        #[doc = "(Deprecated) Overall relevancy score for the image."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
        #[doc = "The result image URL."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1WebDetectionWebImage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1WebDetectionWebLabel {
        #[doc = "Label for extra metadata."]
        #[serde(rename = "label", default)]
        pub label: ::std::option::Option<String>,
        #[doc = "The BCP-47 language code for `label`, such as \"en-US\" or \"sr-Latn\".\nFor more information, see\nhttp://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1WebDetectionWebLabel {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1WebDetectionWebPage {
        #[doc = "Fully matching images on the page.\nCan include resized copies of the query image."]
        #[serde(rename = "fullMatchingImages", default)]
        pub full_matching_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P2Beta1WebDetectionWebImage>,
        >,
        #[doc = "Title for the web page, may contain HTML markups."]
        #[serde(rename = "pageTitle", default)]
        pub page_title: ::std::option::Option<String>,
        #[doc = "Partial matching images on the page.\nThose images are similar enough to share some key-point features. For\nexample an original image will likely have partial matching for its\ncrops."]
        #[serde(rename = "partialMatchingImages", default)]
        pub partial_matching_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P2Beta1WebDetectionWebImage>,
        >,
        #[doc = "(Deprecated) Overall relevancy score for the web page."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
        #[doc = "The result web page URL."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1WebDetectionWebPage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P2Beta1Word {
        #[doc = "The bounding box for the word.\nThe vertices are in the order of top-left, top-right, bottom-right,\nbottom-left. When a rotation of the bounding box is detected the rotation\nis represented as around the top-left corner as defined when the text is\nread in the 'natural' orientation.\nFor example:\n\n* when the text is horizontal it might look like:\n  0----1\n  |    |\n  3----2\n* when it's rotated 180 degrees around the top-left corner it becomes:\n  2----3\n  |    |\n  1----0\n  and the vertex order will still be (0, 1, 2, 3)."]
        #[serde(rename = "boundingBox", default)]
        pub bounding_box:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P2Beta1BoundingPoly>,
        #[doc = "Confidence of the OCR results for the word. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Additional information detected for the word."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P2Beta1TextAnnotationTextProperty,
        >,
        #[doc = "List of symbols in the word.\nThe order of the symbols follows the natural reading order."]
        #[serde(rename = "symbols", default)]
        pub symbols: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P2Beta1Symbol>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P2Beta1Word {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVisionV1P3Beta1AnnotateFileResponse {
        #[doc = "Information about the file for which this response is generated."]
        #[serde(rename = "inputConfig", default)]
        pub input_config:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P3Beta1InputConfig>,
        #[doc = "Individual responses to images found within the file. This field will be\nempty if the `error` field is set."]
        #[serde(rename = "responses", default)]
        pub responses: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P3Beta1AnnotateImageResponse>,
        >,
        #[doc = "This field gives the total number of pages in the file."]
        #[serde(rename = "totalPages", default)]
        pub total_pages: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1AnnotateFileResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVisionV1P3Beta1AnnotateImageResponse {
        #[doc = "If present, contextual information is needed to understand where this image\ncomes from."]
        #[serde(rename = "context", default)]
        pub context:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P3Beta1ImageAnnotationContext>,
        #[doc = "If present, crop hints have completed successfully."]
        #[serde(rename = "cropHintsAnnotation", default)]
        pub crop_hints_annotation:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P3Beta1CropHintsAnnotation>,
        #[doc = "If set, represents the error message for the operation.\nNote that filled-in image annotations are guaranteed to be\ncorrect, even when `error` is set."]
        #[serde(rename = "error", default)]
        pub error: ::std::option::Option<crate::schemas::Status>,
        #[doc = "If present, face detection has completed successfully."]
        #[serde(rename = "faceAnnotations", default)]
        pub face_annotations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P3Beta1FaceAnnotation>>,
        #[doc = "If present, text (OCR) detection or document (OCR) text detection has\ncompleted successfully.\nThis annotation provides the structural hierarchy for the OCR detected\ntext."]
        #[serde(rename = "fullTextAnnotation", default)]
        pub full_text_annotation:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P3Beta1TextAnnotation>,
        #[doc = "If present, image properties were extracted successfully."]
        #[serde(rename = "imagePropertiesAnnotation", default)]
        pub image_properties_annotation:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P3Beta1ImageProperties>,
        #[doc = "If present, label detection has completed successfully."]
        #[serde(rename = "labelAnnotations", default)]
        pub label_annotations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P3Beta1EntityAnnotation>>,
        #[doc = "If present, landmark detection has completed successfully."]
        #[serde(rename = "landmarkAnnotations", default)]
        pub landmark_annotations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P3Beta1EntityAnnotation>>,
        #[doc = "If present, localized object detection has completed successfully.\nThis will be sorted descending by confidence score."]
        #[serde(rename = "localizedObjectAnnotations", default)]
        pub localized_object_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P3Beta1LocalizedObjectAnnotation>,
        >,
        #[doc = "If present, logo detection has completed successfully."]
        #[serde(rename = "logoAnnotations", default)]
        pub logo_annotations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P3Beta1EntityAnnotation>>,
        #[doc = "If present, product search has completed successfully."]
        #[serde(rename = "productSearchResults", default)]
        pub product_search_results:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P3Beta1ProductSearchResults>,
        #[doc = "If present, safe-search annotation has completed successfully."]
        #[serde(rename = "safeSearchAnnotation", default)]
        pub safe_search_annotation:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P3Beta1SafeSearchAnnotation>,
        #[doc = "If present, text (OCR) detection has completed successfully."]
        #[serde(rename = "textAnnotations", default)]
        pub text_annotations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P3Beta1EntityAnnotation>>,
        #[doc = "If present, web detection has completed successfully."]
        #[serde(rename = "webDetection", default)]
        pub web_detection:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P3Beta1WebDetection>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1AnnotateImageResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1AsyncAnnotateFileResponse {
        #[doc = "The output location and metadata from AsyncAnnotateFileRequest."]
        #[serde(rename = "outputConfig", default)]
        pub output_config:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P3Beta1OutputConfig>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1AsyncAnnotateFileResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1AsyncBatchAnnotateFilesResponse {
        #[doc = "The list of file annotation responses, one for each request in\nAsyncBatchAnnotateFilesRequest."]
        #[serde(rename = "responses", default)]
        pub responses: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P3Beta1AsyncAnnotateFileResponse>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1AsyncBatchAnnotateFilesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1BatchOperationMetadata {
        #[doc = "The time when the batch request is finished and\ngoogle.longrunning.Operation.done is set to true."]
        #[serde(rename = "endTime", default)]
        pub end_time: ::std::option::Option<String>,
        #[doc = "The current state of the batch operation."]
        #[serde(rename = "state", default)]
        pub state: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P3Beta1BatchOperationMetadataState,
        >,
        #[doc = "The time when the batch request was submitted to the server."]
        #[serde(rename = "submitTime", default)]
        pub submit_time: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1BatchOperationMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P3Beta1BatchOperationMetadataState {
        #[doc = "The request is done after the longrunning.Operations.CancelOperation has\nbeen called by the user.  Any records that were processed before the\ncancel command are output as specified in the request."]
        Cancelled,
        #[doc = "The request is done and no item has been successfully processed."]
        Failed,
        #[doc = "Request is actively being processed."]
        Processing,
        #[doc = "Invalid."]
        StateUnspecified,
        #[doc = "The request is done and at least one item has been successfully\nprocessed."]
        Successful,
    }
    impl GoogleCloudVisionV1P3Beta1BatchOperationMetadataState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P3Beta1BatchOperationMetadataState::Cancelled => "CANCELLED",
                GoogleCloudVisionV1P3Beta1BatchOperationMetadataState::Failed => "FAILED",
                GoogleCloudVisionV1P3Beta1BatchOperationMetadataState::Processing => "PROCESSING",
                GoogleCloudVisionV1P3Beta1BatchOperationMetadataState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
                GoogleCloudVisionV1P3Beta1BatchOperationMetadataState::Successful => "SUCCESSFUL",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P3Beta1BatchOperationMetadataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P3Beta1BatchOperationMetadataState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P3Beta1BatchOperationMetadataState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => GoogleCloudVisionV1P3Beta1BatchOperationMetadataState::Cancelled,
                "FAILED" => GoogleCloudVisionV1P3Beta1BatchOperationMetadataState::Failed,
                "PROCESSING" => GoogleCloudVisionV1P3Beta1BatchOperationMetadataState::Processing,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudVisionV1P3Beta1BatchOperationMetadataState::StateUnspecified
                }
                "SUCCESSFUL" => GoogleCloudVisionV1P3Beta1BatchOperationMetadataState::Successful,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1BatchOperationMetadataState {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1Block {
        #[doc = "Detected block type (text, image etc) for this block."]
        #[serde(rename = "blockType", default)]
        pub block_type:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P3Beta1BlockBlockType>,
        #[doc = "The bounding box for the block.\nThe vertices are in the order of top-left, top-right, bottom-right,\nbottom-left. When a rotation of the bounding box is detected the rotation\nis represented as around the top-left corner as defined when the text is\nread in the 'natural' orientation.\nFor example:\n\n* when the text is horizontal it might look like:\n  \n  ````text\n    0----1\n    |    |\n    3----2\n  ````\n\n* when it's rotated 180 degrees around the top-left corner it becomes:\n  \n  ````text\n    2----3\n    |    |\n    1----0\n  ````\n  \n  and the vertex order will still be (0, 1, 2, 3)."]
        #[serde(rename = "boundingBox", default)]
        pub bounding_box:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P3Beta1BoundingPoly>,
        #[doc = "Confidence of the OCR results on the block. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "List of paragraphs in this block (if this blocks is of type text)."]
        #[serde(rename = "paragraphs", default)]
        pub paragraphs:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P3Beta1Paragraph>>,
        #[doc = "Additional information detected for the block."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P3Beta1TextAnnotationTextProperty,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1Block {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P3Beta1BlockBlockType {
        #[doc = "Barcode block."]
        Barcode,
        #[doc = "Image block."]
        Picture,
        #[doc = "Horizontal/vertical line box."]
        Ruler,
        #[doc = "Table block."]
        Table,
        #[doc = "Regular text block."]
        Text,
        #[doc = "Unknown block type."]
        Unknown,
    }
    impl GoogleCloudVisionV1P3Beta1BlockBlockType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P3Beta1BlockBlockType::Barcode => "BARCODE",
                GoogleCloudVisionV1P3Beta1BlockBlockType::Picture => "PICTURE",
                GoogleCloudVisionV1P3Beta1BlockBlockType::Ruler => "RULER",
                GoogleCloudVisionV1P3Beta1BlockBlockType::Table => "TABLE",
                GoogleCloudVisionV1P3Beta1BlockBlockType::Text => "TEXT",
                GoogleCloudVisionV1P3Beta1BlockBlockType::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P3Beta1BlockBlockType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P3Beta1BlockBlockType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P3Beta1BlockBlockType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BARCODE" => GoogleCloudVisionV1P3Beta1BlockBlockType::Barcode,
                "PICTURE" => GoogleCloudVisionV1P3Beta1BlockBlockType::Picture,
                "RULER" => GoogleCloudVisionV1P3Beta1BlockBlockType::Ruler,
                "TABLE" => GoogleCloudVisionV1P3Beta1BlockBlockType::Table,
                "TEXT" => GoogleCloudVisionV1P3Beta1BlockBlockType::Text,
                "UNKNOWN" => GoogleCloudVisionV1P3Beta1BlockBlockType::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1BlockBlockType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1BoundingPoly {
        #[doc = "The bounding polygon normalized vertices."]
        #[serde(rename = "normalizedVertices", default)]
        pub normalized_vertices:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P3Beta1NormalizedVertex>>,
        #[doc = "The bounding polygon vertices."]
        #[serde(rename = "vertices", default)]
        pub vertices: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P3Beta1Vertex>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1BoundingPoly {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1ColorInfo {
        #[doc = "RGB components of the color."]
        #[serde(rename = "color", default)]
        pub color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The fraction of pixels the color occupies in the image.\nValue in range [0, 1]."]
        #[serde(rename = "pixelFraction", default)]
        pub pixel_fraction: ::std::option::Option<f32>,
        #[doc = "Image-specific score for this color. Value in range [0, 1]."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1ColorInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1CropHint {
        #[doc = "The bounding polygon for the crop region. The coordinates of the bounding\nbox are in the original image's scale."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P3Beta1BoundingPoly>,
        #[doc = "Confidence of this being a salient region.  Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Fraction of importance of this salient region with respect to the original\nimage."]
        #[serde(rename = "importanceFraction", default)]
        pub importance_fraction: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1CropHint {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1CropHintsAnnotation {
        #[doc = "Crop hint results."]
        #[serde(rename = "cropHints", default)]
        pub crop_hints:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P3Beta1CropHint>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1CropHintsAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1DominantColorsAnnotation {
        #[doc = "RGB color values with their score and pixel fraction."]
        #[serde(rename = "colors", default)]
        pub colors: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P3Beta1ColorInfo>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1DominantColorsAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1EntityAnnotation {
        #[doc = "Image region to which this entity belongs. Not produced\nfor `LABEL_DETECTION` features."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P3Beta1BoundingPoly>,
        #[doc = "**Deprecated. Use `score` instead.**\nThe accuracy of the entity detection in an image.\nFor example, for an image in which the \"Eiffel Tower\" entity is detected,\nthis field represents the confidence that there is a tower in the query\nimage. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Entity textual description, expressed in its `locale` language."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "The language code for the locale in which the entity textual\n`description` is expressed."]
        #[serde(rename = "locale", default)]
        pub locale: ::std::option::Option<String>,
        #[doc = "The location information for the detected entity. Multiple\n`LocationInfo` elements can be present because one location may\nindicate the location of the scene in the image, and another location\nmay indicate the location of the place where the image was taken.\nLocation information is usually present for landmarks."]
        #[serde(rename = "locations", default)]
        pub locations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P3Beta1LocationInfo>>,
        #[doc = "Opaque entity ID. Some IDs may be available in\n[Google Knowledge Graph Search\nAPI](https://developers.google.com/knowledge-graph/)."]
        #[serde(rename = "mid", default)]
        pub mid: ::std::option::Option<String>,
        #[doc = "Some entities may have optional user-supplied `Property` (name/value)\nfields, such a score or string that qualifies the entity."]
        #[serde(rename = "properties", default)]
        pub properties:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P3Beta1Property>>,
        #[doc = "Overall score of the result. Range [0, 1]."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
        #[doc = "The relevancy of the ICA (Image Content Annotation) label to the\nimage. For example, the relevancy of \"tower\" is likely higher to an image\ncontaining the detected \"Eiffel Tower\" than to an image containing a\ndetected distant towering building, even though the confidence that\nthere is a tower in each image may be the same. Range [0, 1]."]
        #[serde(rename = "topicality", default)]
        pub topicality: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1EntityAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1FaceAnnotation {
        #[doc = "Anger likelihood."]
        #[serde(rename = "angerLikelihood", default)]
        pub anger_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P3Beta1FaceAnnotationAngerLikelihood,
        >,
        #[doc = "Blurred likelihood."]
        #[serde(rename = "blurredLikelihood", default)]
        pub blurred_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P3Beta1FaceAnnotationBlurredLikelihood,
        >,
        #[doc = "The bounding polygon around the face. The coordinates of the bounding box\nare in the original image's scale.\nThe bounding box is computed to \"frame\" the face in accordance with human\nexpectations. It is based on the landmarker results.\nNote that one or more x and/or y coordinates may not be generated in the\n`BoundingPoly` (the polygon will be unbounded) if only a partial face\nappears in the image to be annotated."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P3Beta1BoundingPoly>,
        #[doc = "Detection confidence. Range [0, 1]."]
        #[serde(rename = "detectionConfidence", default)]
        pub detection_confidence: ::std::option::Option<f32>,
        #[doc = "The `fd_bounding_poly` bounding polygon is tighter than the\n`boundingPoly`, and encloses only the skin part of the face. Typically, it\nis used to eliminate the face from any image analysis that detects the\n\"amount of skin\" visible in an image. It is not based on the\nlandmarker results, only on the initial face detection, hence\nthe <code>fd</code> (face detection) prefix."]
        #[serde(rename = "fdBoundingPoly", default)]
        pub fd_bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P3Beta1BoundingPoly>,
        #[doc = "Headwear likelihood."]
        #[serde(rename = "headwearLikelihood", default)]
        pub headwear_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P3Beta1FaceAnnotationHeadwearLikelihood,
        >,
        #[doc = "Joy likelihood."]
        #[serde(rename = "joyLikelihood", default)]
        pub joy_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P3Beta1FaceAnnotationJoyLikelihood,
        >,
        #[doc = "Face landmarking confidence. Range [0, 1]."]
        #[serde(rename = "landmarkingConfidence", default)]
        pub landmarking_confidence: ::std::option::Option<f32>,
        #[doc = "Detected face landmarks."]
        #[serde(rename = "landmarks", default)]
        pub landmarks: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P3Beta1FaceAnnotationLandmark>,
        >,
        #[doc = "Yaw angle, which indicates the leftward/rightward angle that the face is\npointing relative to the vertical plane perpendicular to the image. Range\n[-180,180]."]
        #[serde(rename = "panAngle", default)]
        pub pan_angle: ::std::option::Option<f32>,
        #[doc = "Roll angle, which indicates the amount of clockwise/anti-clockwise rotation\nof the face relative to the image vertical about the axis perpendicular to\nthe face. Range [-180,180]."]
        #[serde(rename = "rollAngle", default)]
        pub roll_angle: ::std::option::Option<f32>,
        #[doc = "Sorrow likelihood."]
        #[serde(rename = "sorrowLikelihood", default)]
        pub sorrow_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P3Beta1FaceAnnotationSorrowLikelihood,
        >,
        #[doc = "Surprise likelihood."]
        #[serde(rename = "surpriseLikelihood", default)]
        pub surprise_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P3Beta1FaceAnnotationSurpriseLikelihood,
        >,
        #[doc = "Pitch angle, which indicates the upwards/downwards angle that the face is\npointing relative to the image's horizontal plane. Range [-180,180]."]
        #[serde(rename = "tiltAngle", default)]
        pub tilt_angle: ::std::option::Option<f32>,
        #[doc = "Under-exposed likelihood."]
        #[serde(rename = "underExposedLikelihood", default)]
        pub under_exposed_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P3Beta1FaceAnnotationUnderExposedLikelihood,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1FaceAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P3Beta1FaceAnnotationAngerLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P3Beta1FaceAnnotationAngerLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P3Beta1FaceAnnotationAngerLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P3Beta1FaceAnnotationAngerLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P3Beta1FaceAnnotationAngerLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P3Beta1FaceAnnotationAngerLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P3Beta1FaceAnnotationAngerLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationAngerLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P3Beta1FaceAnnotationAngerLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P3Beta1FaceAnnotationAngerLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P3Beta1FaceAnnotationAngerLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P3Beta1FaceAnnotationAngerLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P3Beta1FaceAnnotationAngerLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P3Beta1FaceAnnotationAngerLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P3Beta1FaceAnnotationAngerLikelihood::Unlikely,
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationAngerLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationAngerLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1FaceAnnotationAngerLikelihood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P3Beta1FaceAnnotationBlurredLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P3Beta1FaceAnnotationBlurredLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P3Beta1FaceAnnotationBlurredLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P3Beta1FaceAnnotationBlurredLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P3Beta1FaceAnnotationBlurredLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P3Beta1FaceAnnotationBlurredLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P3Beta1FaceAnnotationBlurredLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationBlurredLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P3Beta1FaceAnnotationBlurredLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P3Beta1FaceAnnotationBlurredLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P3Beta1FaceAnnotationBlurredLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P3Beta1FaceAnnotationBlurredLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P3Beta1FaceAnnotationBlurredLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P3Beta1FaceAnnotationBlurredLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P3Beta1FaceAnnotationBlurredLikelihood::Unlikely,
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationBlurredLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationBlurredLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1FaceAnnotationBlurredLikelihood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P3Beta1FaceAnnotationHeadwearLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P3Beta1FaceAnnotationHeadwearLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P3Beta1FaceAnnotationHeadwearLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P3Beta1FaceAnnotationHeadwearLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P3Beta1FaceAnnotationHeadwearLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P3Beta1FaceAnnotationHeadwearLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P3Beta1FaceAnnotationHeadwearLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationHeadwearLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P3Beta1FaceAnnotationHeadwearLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P3Beta1FaceAnnotationHeadwearLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P3Beta1FaceAnnotationHeadwearLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P3Beta1FaceAnnotationHeadwearLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P3Beta1FaceAnnotationHeadwearLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P3Beta1FaceAnnotationHeadwearLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P3Beta1FaceAnnotationHeadwearLikelihood::Unlikely,
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationHeadwearLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationHeadwearLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVisionV1P3Beta1FaceAnnotationHeadwearLikelihood
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P3Beta1FaceAnnotationJoyLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P3Beta1FaceAnnotationJoyLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P3Beta1FaceAnnotationJoyLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P3Beta1FaceAnnotationJoyLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P3Beta1FaceAnnotationJoyLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P3Beta1FaceAnnotationJoyLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P3Beta1FaceAnnotationJoyLikelihood::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P3Beta1FaceAnnotationJoyLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P3Beta1FaceAnnotationJoyLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P3Beta1FaceAnnotationJoyLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P3Beta1FaceAnnotationJoyLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P3Beta1FaceAnnotationJoyLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P3Beta1FaceAnnotationJoyLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P3Beta1FaceAnnotationJoyLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P3Beta1FaceAnnotationJoyLikelihood::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P3Beta1FaceAnnotationJoyLikelihood::VeryLikely,
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationJoyLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1FaceAnnotationJoyLikelihood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P3Beta1FaceAnnotationSorrowLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P3Beta1FaceAnnotationSorrowLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P3Beta1FaceAnnotationSorrowLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P3Beta1FaceAnnotationSorrowLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P3Beta1FaceAnnotationSorrowLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P3Beta1FaceAnnotationSorrowLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P3Beta1FaceAnnotationSorrowLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationSorrowLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P3Beta1FaceAnnotationSorrowLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P3Beta1FaceAnnotationSorrowLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P3Beta1FaceAnnotationSorrowLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P3Beta1FaceAnnotationSorrowLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P3Beta1FaceAnnotationSorrowLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P3Beta1FaceAnnotationSorrowLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P3Beta1FaceAnnotationSorrowLikelihood::Unlikely,
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationSorrowLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationSorrowLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1FaceAnnotationSorrowLikelihood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P3Beta1FaceAnnotationSurpriseLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P3Beta1FaceAnnotationSurpriseLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P3Beta1FaceAnnotationSurpriseLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P3Beta1FaceAnnotationSurpriseLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P3Beta1FaceAnnotationSurpriseLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P3Beta1FaceAnnotationSurpriseLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P3Beta1FaceAnnotationSurpriseLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationSurpriseLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P3Beta1FaceAnnotationSurpriseLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P3Beta1FaceAnnotationSurpriseLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P3Beta1FaceAnnotationSurpriseLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P3Beta1FaceAnnotationSurpriseLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P3Beta1FaceAnnotationSurpriseLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P3Beta1FaceAnnotationSurpriseLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P3Beta1FaceAnnotationSurpriseLikelihood::Unlikely,
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationSurpriseLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationSurpriseLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVisionV1P3Beta1FaceAnnotationSurpriseLikelihood
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P3Beta1FaceAnnotationUnderExposedLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P3Beta1FaceAnnotationUnderExposedLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P3Beta1FaceAnnotationUnderExposedLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P3Beta1FaceAnnotationUnderExposedLikelihood::Possible => {
                    "POSSIBLE"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationUnderExposedLikelihood::Unknown => {
                    "UNKNOWN"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationUnderExposedLikelihood::Unlikely => {
                    "UNLIKELY"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationUnderExposedLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationUnderExposedLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P3Beta1FaceAnnotationUnderExposedLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P3Beta1FaceAnnotationUnderExposedLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVisionV1P3Beta1FaceAnnotationUnderExposedLikelihood
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P3Beta1FaceAnnotationUnderExposedLikelihood::Likely,
                "POSSIBLE" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationUnderExposedLikelihood::Possible
                }
                "UNKNOWN" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationUnderExposedLikelihood::Unknown
                }
                "UNLIKELY" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationUnderExposedLikelihood::Unlikely
                }
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationUnderExposedLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationUnderExposedLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVisionV1P3Beta1FaceAnnotationUnderExposedLikelihood
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1FaceAnnotationLandmark {
        #[doc = "Face landmark position."]
        #[serde(rename = "position", default)]
        pub position: ::std::option::Option<crate::schemas::GoogleCloudVisionV1P3Beta1Position>,
        #[doc = "Face landmark type."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1FaceAnnotationLandmark {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType {
        #[doc = "Chin gnathion."]
        ChinGnathion,
        #[doc = "Chin left gonion."]
        ChinLeftGonion,
        #[doc = "Chin right gonion."]
        ChinRightGonion,
        #[doc = "Forehead glabella."]
        ForeheadGlabella,
        #[doc = "Left ear tragion."]
        LeftEarTragion,
        #[doc = "Left eye."]
        LeftEye,
        #[doc = "Left eye, bottom boundary."]
        LeftEyeBottomBoundary,
        #[doc = "Left eye, left corner."]
        LeftEyeLeftCorner,
        #[doc = "Left eye pupil."]
        LeftEyePupil,
        #[doc = "Left eye, right corner."]
        LeftEyeRightCorner,
        #[doc = "Left eye, top boundary."]
        LeftEyeTopBoundary,
        #[doc = "Left eyebrow, upper midpoint."]
        LeftEyebrowUpperMidpoint,
        #[doc = "Left of left eyebrow."]
        LeftOfLeftEyebrow,
        #[doc = "Left of right eyebrow."]
        LeftOfRightEyebrow,
        #[doc = "Lower lip."]
        LowerLip,
        #[doc = "Midpoint between eyes."]
        MidpointBetweenEyes,
        #[doc = "Mouth center."]
        MouthCenter,
        #[doc = "Mouth left."]
        MouthLeft,
        #[doc = "Mouth right."]
        MouthRight,
        #[doc = "Nose, bottom center."]
        NoseBottomCenter,
        #[doc = "Nose, bottom left."]
        NoseBottomLeft,
        #[doc = "Nose, bottom right."]
        NoseBottomRight,
        #[doc = "Nose tip."]
        NoseTip,
        #[doc = "Right ear tragion."]
        RightEarTragion,
        #[doc = "Right eye."]
        RightEye,
        #[doc = "Right eye, bottom boundary."]
        RightEyeBottomBoundary,
        #[doc = "Right eye, left corner."]
        RightEyeLeftCorner,
        #[doc = "Right eye pupil."]
        RightEyePupil,
        #[doc = "Right eye, right corner."]
        RightEyeRightCorner,
        #[doc = "Right eye, top boundary."]
        RightEyeTopBoundary,
        #[doc = "Right eyebrow, upper midpoint."]
        RightEyebrowUpperMidpoint,
        #[doc = "Right of left eyebrow."]
        RightOfLeftEyebrow,
        #[doc = "Right of right eyebrow."]
        RightOfRightEyebrow,
        #[doc = "Unknown face landmark detected. Should not be filled."]
        UnknownLandmark,
        #[doc = "Upper lip."]
        UpperLip,
    }
    impl GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::ChinGnathion => {
                    "CHIN_GNATHION"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::ChinLeftGonion => {
                    "CHIN_LEFT_GONION"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::ChinRightGonion => {
                    "CHIN_RIGHT_GONION"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::ForeheadGlabella => {
                    "FOREHEAD_GLABELLA"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::LeftEarTragion => {
                    "LEFT_EAR_TRAGION"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::LeftEye => "LEFT_EYE",
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::LeftEyeBottomBoundary => {
                    "LEFT_EYE_BOTTOM_BOUNDARY"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::LeftEyeLeftCorner => {
                    "LEFT_EYE_LEFT_CORNER"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::LeftEyePupil => {
                    "LEFT_EYE_PUPIL"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::LeftEyeRightCorner => {
                    "LEFT_EYE_RIGHT_CORNER"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::LeftEyeTopBoundary => {
                    "LEFT_EYE_TOP_BOUNDARY"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::LeftEyebrowUpperMidpoint => {
                    "LEFT_EYEBROW_UPPER_MIDPOINT"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::LeftOfLeftEyebrow => {
                    "LEFT_OF_LEFT_EYEBROW"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::LeftOfRightEyebrow => {
                    "LEFT_OF_RIGHT_EYEBROW"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::LowerLip => "LOWER_LIP",
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::MidpointBetweenEyes => {
                    "MIDPOINT_BETWEEN_EYES"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::MouthCenter => "MOUTH_CENTER",
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::MouthLeft => "MOUTH_LEFT",
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::MouthRight => "MOUTH_RIGHT",
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::NoseBottomCenter => {
                    "NOSE_BOTTOM_CENTER"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::NoseBottomLeft => {
                    "NOSE_BOTTOM_LEFT"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::NoseBottomRight => {
                    "NOSE_BOTTOM_RIGHT"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::NoseTip => "NOSE_TIP",
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::RightEarTragion => {
                    "RIGHT_EAR_TRAGION"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::RightEye => "RIGHT_EYE",
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::RightEyeBottomBoundary => {
                    "RIGHT_EYE_BOTTOM_BOUNDARY"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::RightEyeLeftCorner => {
                    "RIGHT_EYE_LEFT_CORNER"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::RightEyePupil => {
                    "RIGHT_EYE_PUPIL"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::RightEyeRightCorner => {
                    "RIGHT_EYE_RIGHT_CORNER"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::RightEyeTopBoundary => {
                    "RIGHT_EYE_TOP_BOUNDARY"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::RightEyebrowUpperMidpoint => {
                    "RIGHT_EYEBROW_UPPER_MIDPOINT"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::RightOfLeftEyebrow => {
                    "RIGHT_OF_LEFT_EYEBROW"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::RightOfRightEyebrow => {
                    "RIGHT_OF_RIGHT_EYEBROW"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::UnknownLandmark => {
                    "UNKNOWN_LANDMARK"
                }
                GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::UpperLip => "UPPER_LIP",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CHIN_GNATHION" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::ChinGnathion
                }
                "CHIN_LEFT_GONION" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::ChinLeftGonion
                }
                "CHIN_RIGHT_GONION" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::ChinRightGonion
                }
                "FOREHEAD_GLABELLA" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::ForeheadGlabella
                }
                "LEFT_EAR_TRAGION" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::LeftEarTragion
                }
                "LEFT_EYE" => GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::LeftEye,
                "LEFT_EYE_BOTTOM_BOUNDARY" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::LeftEyeBottomBoundary
                }
                "LEFT_EYE_LEFT_CORNER" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::LeftEyeLeftCorner
                }
                "LEFT_EYE_PUPIL" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::LeftEyePupil
                }
                "LEFT_EYE_RIGHT_CORNER" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::LeftEyeRightCorner
                }
                "LEFT_EYE_TOP_BOUNDARY" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::LeftEyeTopBoundary
                }
                "LEFT_EYEBROW_UPPER_MIDPOINT" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::LeftEyebrowUpperMidpoint
                }
                "LEFT_OF_LEFT_EYEBROW" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::LeftOfLeftEyebrow
                }
                "LEFT_OF_RIGHT_EYEBROW" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::LeftOfRightEyebrow
                }
                "LOWER_LIP" => GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::LowerLip,
                "MIDPOINT_BETWEEN_EYES" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::MidpointBetweenEyes
                }
                "MOUTH_CENTER" => GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::MouthCenter,
                "MOUTH_LEFT" => GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::MouthLeft,
                "MOUTH_RIGHT" => GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::MouthRight,
                "NOSE_BOTTOM_CENTER" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::NoseBottomCenter
                }
                "NOSE_BOTTOM_LEFT" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::NoseBottomLeft
                }
                "NOSE_BOTTOM_RIGHT" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::NoseBottomRight
                }
                "NOSE_TIP" => GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::NoseTip,
                "RIGHT_EAR_TRAGION" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::RightEarTragion
                }
                "RIGHT_EYE" => GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::RightEye,
                "RIGHT_EYE_BOTTOM_BOUNDARY" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::RightEyeBottomBoundary
                }
                "RIGHT_EYE_LEFT_CORNER" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::RightEyeLeftCorner
                }
                "RIGHT_EYE_PUPIL" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::RightEyePupil
                }
                "RIGHT_EYE_RIGHT_CORNER" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::RightEyeRightCorner
                }
                "RIGHT_EYE_TOP_BOUNDARY" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::RightEyeTopBoundary
                }
                "RIGHT_EYEBROW_UPPER_MIDPOINT" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::RightEyebrowUpperMidpoint
                }
                "RIGHT_OF_LEFT_EYEBROW" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::RightOfLeftEyebrow
                }
                "RIGHT_OF_RIGHT_EYEBROW" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::RightOfRightEyebrow
                }
                "UNKNOWN_LANDMARK" => {
                    GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::UnknownLandmark
                }
                "UPPER_LIP" => GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType::UpperLip,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1FaceAnnotationLandmarkType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1GcsDestination {
        #[doc = "Google Cloud Storage URI prefix where the results will be stored. Results\nwill be in JSON format and preceded by its corresponding input URI prefix.\nThis field can either represent a gcs file prefix or gcs directory. In\neither case, the uri should be unique because in order to get all of the\noutput files, you will need to do a wildcard gcs search on the uri prefix\nyou provide.\n\nExamples:\n\n* File Prefix: gs://bucket-name/here/filenameprefix   The output files\n  will be created in gs://bucket-name/here/ and the names of the\n  output files will begin with \"filenameprefix\".\n\n* Directory Prefix: gs://bucket-name/some/location/   The output files\n  will be created in gs://bucket-name/some/location/ and the names of the\n  output files could be anything because there was no filename prefix\n  specified.\n\nIf multiple outputs, each response is still AnnotateFileResponse, each of\nwhich contains some subset of the full list of AnnotateImageResponse.\nMultiple outputs can happen if, for example, the output JSON is too large\nand overflows into multiple sharded files."]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1GcsDestination {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1GcsSource {
        #[doc = "Google Cloud Storage URI for the input file. This must only be a\nGoogle Cloud Storage object. Wildcards are not currently supported."]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1GcsSource {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1ImageAnnotationContext {
        #[doc = "If the file was a PDF or TIFF, this field gives the page number within\nthe file used to produce the image."]
        #[serde(rename = "pageNumber", default)]
        pub page_number: ::std::option::Option<i32>,
        #[doc = "The URI of the file used to produce the image."]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1ImageAnnotationContext {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1ImageProperties {
        #[doc = "If present, dominant colors completed successfully."]
        #[serde(rename = "dominantColors", default)]
        pub dominant_colors: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P3Beta1DominantColorsAnnotation,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1ImageProperties {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVisionV1P3Beta1ImportProductSetsResponse {
        #[doc = "The list of reference_images that are imported successfully."]
        #[serde(rename = "referenceImages", default)]
        pub reference_images:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P3Beta1ReferenceImage>>,
        #[doc = "The rpc status for each ImportProductSet request, including both successes\nand errors.\n\nThe number of statuses here matches the number of lines in the csv file,\nand statuses[i] stores the success or failure status of processing the i-th\nline of the csv, starting from line 0."]
        #[serde(rename = "statuses", default)]
        pub statuses: ::std::option::Option<Vec<crate::schemas::Status>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1ImportProductSetsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1InputConfig {
        #[doc = "File content, represented as a stream of bytes.\nNote: As with all `bytes` fields, protobuffers use a pure binary\nrepresentation, whereas JSON representations use base64.\n\nCurrently, this field only works for BatchAnnotateFiles requests. It does\nnot work for AsyncBatchAnnotateFiles requests."]
        #[serde(rename = "content", default)]
        pub content: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "The Google Cloud Storage location to read the input from."]
        #[serde(rename = "gcsSource", default)]
        pub gcs_source: ::std::option::Option<crate::schemas::GoogleCloudVisionV1P3Beta1GcsSource>,
        #[doc = "The type of the file. Currently only \"application/pdf\", \"image/tiff\" and\n\"image/gif\" are supported. Wildcards are not supported."]
        #[serde(rename = "mimeType", default)]
        pub mime_type: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1InputConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1LocalizedObjectAnnotation {
        #[doc = "Image region to which this object belongs. This must be populated."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P3Beta1BoundingPoly>,
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more\ninformation, see\nhttp://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Object ID that should align with EntityAnnotation mid."]
        #[serde(rename = "mid", default)]
        pub mid: ::std::option::Option<String>,
        #[doc = "Object name, expressed in its `language_code` language."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Score of the result. Range [0, 1]."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1LocalizedObjectAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1LocationInfo {
        #[doc = "lat/long location coordinates."]
        #[serde(rename = "latLng", default)]
        pub lat_lng: ::std::option::Option<crate::schemas::LatLng>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1LocationInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1NormalizedVertex {
        #[doc = "X coordinate."]
        #[serde(rename = "x", default)]
        pub x: ::std::option::Option<f32>,
        #[doc = "Y coordinate."]
        #[serde(rename = "y", default)]
        pub y: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1NormalizedVertex {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1OperationMetadata {
        #[doc = "The time when the batch request was received."]
        #[serde(rename = "createTime", default)]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Current state of the batch operation."]
        #[serde(rename = "state", default)]
        pub state:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P3Beta1OperationMetadataState>,
        #[doc = "The time when the operation result was last updated."]
        #[serde(rename = "updateTime", default)]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1OperationMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P3Beta1OperationMetadataState {
        #[doc = "The batch processing was cancelled."]
        Cancelled,
        #[doc = "Request is received."]
        Created,
        #[doc = "The batch processing is done."]
        Done,
        #[doc = "Request is actively being processed."]
        Running,
        #[doc = "Invalid."]
        StateUnspecified,
    }
    impl GoogleCloudVisionV1P3Beta1OperationMetadataState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P3Beta1OperationMetadataState::Cancelled => "CANCELLED",
                GoogleCloudVisionV1P3Beta1OperationMetadataState::Created => "CREATED",
                GoogleCloudVisionV1P3Beta1OperationMetadataState::Done => "DONE",
                GoogleCloudVisionV1P3Beta1OperationMetadataState::Running => "RUNNING",
                GoogleCloudVisionV1P3Beta1OperationMetadataState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P3Beta1OperationMetadataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P3Beta1OperationMetadataState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P3Beta1OperationMetadataState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => GoogleCloudVisionV1P3Beta1OperationMetadataState::Cancelled,
                "CREATED" => GoogleCloudVisionV1P3Beta1OperationMetadataState::Created,
                "DONE" => GoogleCloudVisionV1P3Beta1OperationMetadataState::Done,
                "RUNNING" => GoogleCloudVisionV1P3Beta1OperationMetadataState::Running,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudVisionV1P3Beta1OperationMetadataState::StateUnspecified
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1OperationMetadataState {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1OutputConfig {
        #[doc = "The max number of response protos to put into each output JSON file on\nGoogle Cloud Storage.\nThe valid range is [1, 100]. If not specified, the default value is 20.\n\nFor example, for one pdf file with 100 pages, 100 response protos will\nbe generated. If `batch_size` = 20, then 5 json files each\ncontaining 20 response protos will be written under the prefix\n`gcs_destination`.`uri`.\n\nCurrently, batch_size only applies to GcsDestination, with potential future\nsupport for other output configurations."]
        #[serde(rename = "batchSize", default)]
        pub batch_size: ::std::option::Option<i32>,
        #[doc = "The Google Cloud Storage location to write the output(s) to."]
        #[serde(rename = "gcsDestination", default)]
        pub gcs_destination:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P3Beta1GcsDestination>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1OutputConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1Page {
        #[doc = "List of blocks of text, images etc on this page."]
        #[serde(rename = "blocks", default)]
        pub blocks: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P3Beta1Block>>,
        #[doc = "Confidence of the OCR results on the page. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Page height. For PDFs the unit is points. For images (including\nTIFFs) the unit is pixels."]
        #[serde(rename = "height", default)]
        pub height: ::std::option::Option<i32>,
        #[doc = "Additional information detected on the page."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P3Beta1TextAnnotationTextProperty,
        >,
        #[doc = "Page width. For PDFs the unit is points. For images (including\nTIFFs) the unit is pixels."]
        #[serde(rename = "width", default)]
        pub width: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1Page {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1Paragraph {
        #[doc = "The bounding box for the paragraph.\nThe vertices are in the order of top-left, top-right, bottom-right,\nbottom-left. When a rotation of the bounding box is detected the rotation\nis represented as around the top-left corner as defined when the text is\nread in the 'natural' orientation.\nFor example:\n\n* when the text is horizontal it might look like:\n  0----1\n  |    |\n  3----2\n* when it's rotated 180 degrees around the top-left corner it becomes:\n  2----3\n  |    |\n  1----0\n  and the vertex order will still be (0, 1, 2, 3)."]
        #[serde(rename = "boundingBox", default)]
        pub bounding_box:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P3Beta1BoundingPoly>,
        #[doc = "Confidence of the OCR results for the paragraph. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Additional information detected for the paragraph."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P3Beta1TextAnnotationTextProperty,
        >,
        #[doc = "List of all words in this paragraph."]
        #[serde(rename = "words", default)]
        pub words: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P3Beta1Word>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1Paragraph {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1Position {
        #[doc = "X coordinate."]
        #[serde(rename = "x", default)]
        pub x: ::std::option::Option<f32>,
        #[doc = "Y coordinate."]
        #[serde(rename = "y", default)]
        pub y: ::std::option::Option<f32>,
        #[doc = "Z coordinate (or depth)."]
        #[serde(rename = "z", default)]
        pub z: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1Position {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1Product {
        #[doc = "User-provided metadata to be stored with this product. Must be at most 4096\ncharacters long."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "The user-provided name for this Product. Must not be empty. Must be at most\n4096 characters long."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The resource name of the product.\n\nFormat is:\n`projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`.\n\nThis field is ignored when creating a product."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The category for the product identified by the reference image. This should\nbe either \"homegoods-v2\", \"apparel-v2\", or \"toys-v2\". The legacy categories\n\"homegoods\", \"apparel\", and \"toys\" are still supported, but these should\nnot be used for new products.\n\nThis field is immutable."]
        #[serde(rename = "productCategory", default)]
        pub product_category: ::std::option::Option<String>,
        #[doc = "Key-value pairs that can be attached to a product. At query time,\nconstraints can be specified based on the product_labels.\n\nNote that integer values can be provided as strings, e.g. \"1199\". Only\nstrings with integer values can match a range-based restriction which is\nto be supported soon.\n\nMultiple values can be assigned to the same key. One product may have up to\n500 product_labels.\n\nNotice that the total number of distinct product_labels over all products\nin one ProductSet cannot exceed 1M, otherwise the product search pipeline\nwill refuse to work for that ProductSet."]
        #[serde(rename = "productLabels", default)]
        pub product_labels:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P3Beta1ProductKeyValue>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1Product {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1ProductKeyValue {
        #[doc = "The key of the label attached to the product. Cannot be empty and cannot\nexceed 128 bytes."]
        #[serde(rename = "key", default)]
        pub key: ::std::option::Option<String>,
        #[doc = "The value of the label attached to the product. Cannot be empty and\ncannot exceed 128 bytes."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1ProductKeyValue {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1ProductSearchResults {
        #[doc = "Timestamp of the index which provided these results. Products added to the\nproduct set and products removed from the product set after this time are\nnot reflected in the current results."]
        #[serde(rename = "indexTime", default)]
        pub index_time: ::std::option::Option<String>,
        #[doc = "List of results grouped by products detected in the query image. Each entry\ncorresponds to one bounding polygon in the query image, and contains the\nmatching products specific to that region. There may be duplicate product\nmatches in the union of all the per-product results."]
        #[serde(rename = "productGroupedResults", default)]
        pub product_grouped_results: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P3Beta1ProductSearchResultsGroupedResult>,
        >,
        #[doc = "List of results, one for each product match."]
        #[serde(rename = "results", default)]
        pub results: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P3Beta1ProductSearchResultsResult>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1ProductSearchResults {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1ProductSearchResultsGroupedResult {
        #[doc = "The bounding polygon around the product detected in the query image."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P3Beta1BoundingPoly>,
        #[doc = "List of results, one for each product match."]
        #[serde(rename = "results", default)]
        pub results: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P3Beta1ProductSearchResultsResult>,
        >,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVisionV1P3Beta1ProductSearchResultsGroupedResult
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1ProductSearchResultsResult {
        #[doc = "The resource name of the image from the product that is the closest match\nto the query."]
        #[serde(rename = "image", default)]
        pub image: ::std::option::Option<String>,
        #[doc = "The Product."]
        #[serde(rename = "product", default)]
        pub product: ::std::option::Option<crate::schemas::GoogleCloudVisionV1P3Beta1Product>,
        #[doc = "A confidence level on the match, ranging from 0 (no confidence) to\n1 (full confidence)."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1ProductSearchResultsResult {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1Property {
        #[doc = "Name of the property."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Value of numeric properties."]
        #[serde(rename = "uint64Value", default)]
        #[serde(with = "crate::parsed_string")]
        pub uint_64_value: ::std::option::Option<u64>,
        #[doc = "Value of the property."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1Property {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1ReferenceImage {
        #[doc = "Bounding polygons around the areas of interest in the reference image.\nOptional. If this field is empty, the system will try to detect regions of\ninterest. At most 10 bounding polygons will be used.\n\nThe provided shape is converted into a non-rotated rectangle. Once\nconverted, the small edge of the rectangle must be greater than or equal\nto 300 pixels. The aspect ratio must be 1:4 or less (i.e. 1:3 is ok; 1:5\nis not)."]
        #[serde(rename = "boundingPolys", default)]
        pub bounding_polys:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P3Beta1BoundingPoly>>,
        #[doc = "The resource name of the reference image.\n\nFormat is:\n\n`projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID/referenceImages/IMAGE_ID`.\n\nThis field is ignored when creating a reference image."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The Google Cloud Storage URI of the reference image.\n\nThe URI must start with `gs://`.\n\nRequired."]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1ReferenceImage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1SafeSearchAnnotation {
        #[doc = "Represents the adult content likelihood for the image. Adult content may\ncontain elements such as nudity, pornographic images or cartoons, or\nsexual activities."]
        #[serde(rename = "adult", default)]
        pub adult: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P3Beta1SafeSearchAnnotationAdult,
        >,
        #[doc = "Likelihood that this is a medical image."]
        #[serde(rename = "medical", default)]
        pub medical: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P3Beta1SafeSearchAnnotationMedical,
        >,
        #[doc = "Likelihood that the request image contains racy content. Racy content may\ninclude (but is not limited to) skimpy or sheer clothing, strategically\ncovered nudity, lewd or provocative poses, or close-ups of sensitive\nbody areas."]
        #[serde(rename = "racy", default)]
        pub racy: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P3Beta1SafeSearchAnnotationRacy,
        >,
        #[doc = "Spoof likelihood. The likelihood that an modification\nwas made to the image's canonical version to make it appear\nfunny or offensive."]
        #[serde(rename = "spoof", default)]
        pub spoof: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P3Beta1SafeSearchAnnotationSpoof,
        >,
        #[doc = "Likelihood that this image contains violent content."]
        #[serde(rename = "violence", default)]
        pub violence: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P3Beta1SafeSearchAnnotationViolence,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1SafeSearchAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P3Beta1SafeSearchAnnotationAdult {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P3Beta1SafeSearchAnnotationAdult {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationAdult::Likely => "LIKELY",
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationAdult::Possible => "POSSIBLE",
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationAdult::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationAdult::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationAdult::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationAdult::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P3Beta1SafeSearchAnnotationAdult {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P3Beta1SafeSearchAnnotationAdult {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P3Beta1SafeSearchAnnotationAdult {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P3Beta1SafeSearchAnnotationAdult::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P3Beta1SafeSearchAnnotationAdult::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P3Beta1SafeSearchAnnotationAdult::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P3Beta1SafeSearchAnnotationAdult::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P3Beta1SafeSearchAnnotationAdult::VeryLikely,
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P3Beta1SafeSearchAnnotationAdult::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1SafeSearchAnnotationAdult {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P3Beta1SafeSearchAnnotationMedical {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P3Beta1SafeSearchAnnotationMedical {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationMedical::Likely => "LIKELY",
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationMedical::Possible => "POSSIBLE",
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationMedical::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationMedical::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationMedical::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationMedical::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P3Beta1SafeSearchAnnotationMedical {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P3Beta1SafeSearchAnnotationMedical {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P3Beta1SafeSearchAnnotationMedical {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P3Beta1SafeSearchAnnotationMedical::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P3Beta1SafeSearchAnnotationMedical::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P3Beta1SafeSearchAnnotationMedical::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P3Beta1SafeSearchAnnotationMedical::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P3Beta1SafeSearchAnnotationMedical::VeryLikely,
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P3Beta1SafeSearchAnnotationMedical::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1SafeSearchAnnotationMedical {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P3Beta1SafeSearchAnnotationRacy {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P3Beta1SafeSearchAnnotationRacy {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationRacy::Likely => "LIKELY",
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationRacy::Possible => "POSSIBLE",
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationRacy::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationRacy::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationRacy::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationRacy::VeryUnlikely => "VERY_UNLIKELY",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P3Beta1SafeSearchAnnotationRacy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P3Beta1SafeSearchAnnotationRacy {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P3Beta1SafeSearchAnnotationRacy {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P3Beta1SafeSearchAnnotationRacy::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P3Beta1SafeSearchAnnotationRacy::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P3Beta1SafeSearchAnnotationRacy::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P3Beta1SafeSearchAnnotationRacy::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P3Beta1SafeSearchAnnotationRacy::VeryLikely,
                "VERY_UNLIKELY" => GoogleCloudVisionV1P3Beta1SafeSearchAnnotationRacy::VeryUnlikely,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1SafeSearchAnnotationRacy {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P3Beta1SafeSearchAnnotationSpoof {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P3Beta1SafeSearchAnnotationSpoof {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationSpoof::Likely => "LIKELY",
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationSpoof::Possible => "POSSIBLE",
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationSpoof::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationSpoof::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationSpoof::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationSpoof::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P3Beta1SafeSearchAnnotationSpoof {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P3Beta1SafeSearchAnnotationSpoof {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P3Beta1SafeSearchAnnotationSpoof {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P3Beta1SafeSearchAnnotationSpoof::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P3Beta1SafeSearchAnnotationSpoof::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P3Beta1SafeSearchAnnotationSpoof::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P3Beta1SafeSearchAnnotationSpoof::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P3Beta1SafeSearchAnnotationSpoof::VeryLikely,
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P3Beta1SafeSearchAnnotationSpoof::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1SafeSearchAnnotationSpoof {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P3Beta1SafeSearchAnnotationViolence {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P3Beta1SafeSearchAnnotationViolence {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationViolence::Likely => "LIKELY",
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationViolence::Possible => "POSSIBLE",
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationViolence::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationViolence::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationViolence::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P3Beta1SafeSearchAnnotationViolence::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P3Beta1SafeSearchAnnotationViolence {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P3Beta1SafeSearchAnnotationViolence {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P3Beta1SafeSearchAnnotationViolence {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P3Beta1SafeSearchAnnotationViolence::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P3Beta1SafeSearchAnnotationViolence::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P3Beta1SafeSearchAnnotationViolence::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P3Beta1SafeSearchAnnotationViolence::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P3Beta1SafeSearchAnnotationViolence::VeryLikely,
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P3Beta1SafeSearchAnnotationViolence::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1SafeSearchAnnotationViolence {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1Symbol {
        #[doc = "The bounding box for the symbol.\nThe vertices are in the order of top-left, top-right, bottom-right,\nbottom-left. When a rotation of the bounding box is detected the rotation\nis represented as around the top-left corner as defined when the text is\nread in the 'natural' orientation.\nFor example:\n\n* when the text is horizontal it might look like:\n  0----1\n  |    |\n  3----2\n* when it's rotated 180 degrees around the top-left corner it becomes:\n  2----3\n  |    |\n  1----0\n  and the vertex order will still be (0, 1, 2, 3)."]
        #[serde(rename = "boundingBox", default)]
        pub bounding_box:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P3Beta1BoundingPoly>,
        #[doc = "Confidence of the OCR results for the symbol. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Additional information detected for the symbol."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P3Beta1TextAnnotationTextProperty,
        >,
        #[doc = "The actual UTF-8 representation of the symbol."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1Symbol {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1TextAnnotation {
        #[doc = "List of pages detected by OCR."]
        #[serde(rename = "pages", default)]
        pub pages: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P3Beta1Page>>,
        #[doc = "UTF-8 text detected on the pages."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1TextAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1TextAnnotationDetectedBreak {
        #[doc = "True if break prepends the element."]
        #[serde(rename = "isPrefix", default)]
        pub is_prefix: ::std::option::Option<bool>,
        #[doc = "Detected break type."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P3Beta1TextAnnotationDetectedBreakType,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1TextAnnotationDetectedBreak {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P3Beta1TextAnnotationDetectedBreakType {
        #[doc = "Line-wrapping break."]
        EolSureSpace,
        #[doc = "End-line hyphen that is not present in text; does not co-occur with\n`SPACE`, `LEADER_SPACE`, or `LINE_BREAK`."]
        Hyphen,
        #[doc = "Line break that ends a paragraph."]
        LineBreak,
        #[doc = "Regular space."]
        Space,
        #[doc = "Sure space (very wide)."]
        SureSpace,
        #[doc = "Unknown break label type."]
        Unknown,
    }
    impl GoogleCloudVisionV1P3Beta1TextAnnotationDetectedBreakType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P3Beta1TextAnnotationDetectedBreakType::EolSureSpace => {
                    "EOL_SURE_SPACE"
                }
                GoogleCloudVisionV1P3Beta1TextAnnotationDetectedBreakType::Hyphen => "HYPHEN",
                GoogleCloudVisionV1P3Beta1TextAnnotationDetectedBreakType::LineBreak => {
                    "LINE_BREAK"
                }
                GoogleCloudVisionV1P3Beta1TextAnnotationDetectedBreakType::Space => "SPACE",
                GoogleCloudVisionV1P3Beta1TextAnnotationDetectedBreakType::SureSpace => {
                    "SURE_SPACE"
                }
                GoogleCloudVisionV1P3Beta1TextAnnotationDetectedBreakType::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P3Beta1TextAnnotationDetectedBreakType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P3Beta1TextAnnotationDetectedBreakType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P3Beta1TextAnnotationDetectedBreakType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EOL_SURE_SPACE" => {
                    GoogleCloudVisionV1P3Beta1TextAnnotationDetectedBreakType::EolSureSpace
                }
                "HYPHEN" => GoogleCloudVisionV1P3Beta1TextAnnotationDetectedBreakType::Hyphen,
                "LINE_BREAK" => {
                    GoogleCloudVisionV1P3Beta1TextAnnotationDetectedBreakType::LineBreak
                }
                "SPACE" => GoogleCloudVisionV1P3Beta1TextAnnotationDetectedBreakType::Space,
                "SURE_SPACE" => {
                    GoogleCloudVisionV1P3Beta1TextAnnotationDetectedBreakType::SureSpace
                }
                "UNKNOWN" => GoogleCloudVisionV1P3Beta1TextAnnotationDetectedBreakType::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1TextAnnotationDetectedBreakType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1TextAnnotationDetectedLanguage {
        #[doc = "Confidence of detected language. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more\ninformation, see\nhttp://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1TextAnnotationDetectedLanguage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1TextAnnotationTextProperty {
        #[doc = "Detected start or end of a text segment."]
        #[serde(rename = "detectedBreak", default)]
        pub detected_break: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P3Beta1TextAnnotationDetectedBreak,
        >,
        #[doc = "A list of detected languages together with confidence."]
        #[serde(rename = "detectedLanguages", default)]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P3Beta1TextAnnotationDetectedLanguage>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1TextAnnotationTextProperty {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1Vertex {
        #[doc = "X coordinate."]
        #[serde(rename = "x", default)]
        pub x: ::std::option::Option<i32>,
        #[doc = "Y coordinate."]
        #[serde(rename = "y", default)]
        pub y: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1Vertex {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1WebDetection {
        #[doc = "The service's best guess as to the topic of the request image.\nInferred from similar images on the open web."]
        #[serde(rename = "bestGuessLabels", default)]
        pub best_guess_labels: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P3Beta1WebDetectionWebLabel>,
        >,
        #[doc = "Fully matching images from the Internet.\nCan include resized copies of the query image."]
        #[serde(rename = "fullMatchingImages", default)]
        pub full_matching_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P3Beta1WebDetectionWebImage>,
        >,
        #[doc = "Web pages containing the matching images from the Internet."]
        #[serde(rename = "pagesWithMatchingImages", default)]
        pub pages_with_matching_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P3Beta1WebDetectionWebPage>,
        >,
        #[doc = "Partial matching images from the Internet.\nThose images are similar enough to share some key-point features. For\nexample an original image will likely have partial matching for its crops."]
        #[serde(rename = "partialMatchingImages", default)]
        pub partial_matching_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P3Beta1WebDetectionWebImage>,
        >,
        #[doc = "The visually similar image results."]
        #[serde(rename = "visuallySimilarImages", default)]
        pub visually_similar_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P3Beta1WebDetectionWebImage>,
        >,
        #[doc = "Deduced entities from similar images on the Internet."]
        #[serde(rename = "webEntities", default)]
        pub web_entities: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P3Beta1WebDetectionWebEntity>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1WebDetection {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1WebDetectionWebEntity {
        #[doc = "Canonical description of the entity, in English."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Opaque entity ID."]
        #[serde(rename = "entityId", default)]
        pub entity_id: ::std::option::Option<String>,
        #[doc = "Overall relevancy score for the entity.\nNot normalized and not comparable across different image queries."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1WebDetectionWebEntity {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1WebDetectionWebImage {
        #[doc = "(Deprecated) Overall relevancy score for the image."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
        #[doc = "The result image URL."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1WebDetectionWebImage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1WebDetectionWebLabel {
        #[doc = "Label for extra metadata."]
        #[serde(rename = "label", default)]
        pub label: ::std::option::Option<String>,
        #[doc = "The BCP-47 language code for `label`, such as \"en-US\" or \"sr-Latn\".\nFor more information, see\nhttp://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1WebDetectionWebLabel {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1WebDetectionWebPage {
        #[doc = "Fully matching images on the page.\nCan include resized copies of the query image."]
        #[serde(rename = "fullMatchingImages", default)]
        pub full_matching_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P3Beta1WebDetectionWebImage>,
        >,
        #[doc = "Title for the web page, may contain HTML markups."]
        #[serde(rename = "pageTitle", default)]
        pub page_title: ::std::option::Option<String>,
        #[doc = "Partial matching images on the page.\nThose images are similar enough to share some key-point features. For\nexample an original image will likely have partial matching for its\ncrops."]
        #[serde(rename = "partialMatchingImages", default)]
        pub partial_matching_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P3Beta1WebDetectionWebImage>,
        >,
        #[doc = "(Deprecated) Overall relevancy score for the web page."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
        #[doc = "The result web page URL."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1WebDetectionWebPage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P3Beta1Word {
        #[doc = "The bounding box for the word.\nThe vertices are in the order of top-left, top-right, bottom-right,\nbottom-left. When a rotation of the bounding box is detected the rotation\nis represented as around the top-left corner as defined when the text is\nread in the 'natural' orientation.\nFor example:\n\n* when the text is horizontal it might look like:\n  0----1\n  |    |\n  3----2\n* when it's rotated 180 degrees around the top-left corner it becomes:\n  2----3\n  |    |\n  1----0\n  and the vertex order will still be (0, 1, 2, 3)."]
        #[serde(rename = "boundingBox", default)]
        pub bounding_box:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P3Beta1BoundingPoly>,
        #[doc = "Confidence of the OCR results for the word. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Additional information detected for the word."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P3Beta1TextAnnotationTextProperty,
        >,
        #[doc = "List of symbols in the word.\nThe order of the symbols follows the natural reading order."]
        #[serde(rename = "symbols", default)]
        pub symbols: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P3Beta1Symbol>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P3Beta1Word {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVisionV1P4Beta1AnnotateFileResponse {
        #[doc = "Information about the file for which this response is generated."]
        #[serde(rename = "inputConfig", default)]
        pub input_config:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P4Beta1InputConfig>,
        #[doc = "Individual responses to images found within the file. This field will be\nempty if the `error` field is set."]
        #[serde(rename = "responses", default)]
        pub responses: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P4Beta1AnnotateImageResponse>,
        >,
        #[doc = "This field gives the total number of pages in the file."]
        #[serde(rename = "totalPages", default)]
        pub total_pages: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1AnnotateFileResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVisionV1P4Beta1AnnotateImageResponse {
        #[doc = "If present, contextual information is needed to understand where this image\ncomes from."]
        #[serde(rename = "context", default)]
        pub context:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P4Beta1ImageAnnotationContext>,
        #[doc = "If present, crop hints have completed successfully."]
        #[serde(rename = "cropHintsAnnotation", default)]
        pub crop_hints_annotation:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P4Beta1CropHintsAnnotation>,
        #[doc = "If set, represents the error message for the operation.\nNote that filled-in image annotations are guaranteed to be\ncorrect, even when `error` is set."]
        #[serde(rename = "error", default)]
        pub error: ::std::option::Option<crate::schemas::Status>,
        #[doc = "If present, face detection has completed successfully."]
        #[serde(rename = "faceAnnotations", default)]
        pub face_annotations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P4Beta1FaceAnnotation>>,
        #[doc = "If present, text (OCR) detection or document (OCR) text detection has\ncompleted successfully.\nThis annotation provides the structural hierarchy for the OCR detected\ntext."]
        #[serde(rename = "fullTextAnnotation", default)]
        pub full_text_annotation:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P4Beta1TextAnnotation>,
        #[doc = "If present, image properties were extracted successfully."]
        #[serde(rename = "imagePropertiesAnnotation", default)]
        pub image_properties_annotation:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P4Beta1ImageProperties>,
        #[doc = "If present, image quality calculation has completed successfully."]
        #[serde(rename = "imageQualityAnnotation", default)]
        pub image_quality_annotation:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P4Beta1ImageQuality>,
        #[doc = "If present, label detection has completed successfully."]
        #[serde(rename = "labelAnnotations", default)]
        pub label_annotations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P4Beta1EntityAnnotation>>,
        #[doc = "If present, landmark detection has completed successfully."]
        #[serde(rename = "landmarkAnnotations", default)]
        pub landmark_annotations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P4Beta1EntityAnnotation>>,
        #[doc = "If present, localized object detection has completed successfully.\nThis will be sorted descending by confidence score."]
        #[serde(rename = "localizedObjectAnnotations", default)]
        pub localized_object_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P4Beta1LocalizedObjectAnnotation>,
        >,
        #[doc = "If present, logo detection has completed successfully."]
        #[serde(rename = "logoAnnotations", default)]
        pub logo_annotations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P4Beta1EntityAnnotation>>,
        #[doc = "If present, product search has completed successfully."]
        #[serde(rename = "productSearchResults", default)]
        pub product_search_results:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P4Beta1ProductSearchResults>,
        #[doc = "If present, image quality optimization has completed successfully."]
        #[serde(rename = "qualityOptimizationResult", default)]
        pub quality_optimization_result: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P4Beta1QualityOptimizationResult,
        >,
        #[doc = "If present, safe-search annotation has completed successfully."]
        #[serde(rename = "safeSearchAnnotation", default)]
        pub safe_search_annotation:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P4Beta1SafeSearchAnnotation>,
        #[doc = "If present, text (OCR) detection has completed successfully."]
        #[serde(rename = "textAnnotations", default)]
        pub text_annotations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P4Beta1EntityAnnotation>>,
        #[doc = "If present, web detection has completed successfully."]
        #[serde(rename = "webDetection", default)]
        pub web_detection:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P4Beta1WebDetection>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1AnnotateImageResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1AsyncAnnotateFileResponse {
        #[doc = "The output location and metadata from AsyncAnnotateFileRequest."]
        #[serde(rename = "outputConfig", default)]
        pub output_config:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P4Beta1OutputConfig>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1AsyncAnnotateFileResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1AsyncBatchAnnotateFilesResponse {
        #[doc = "The list of file annotation responses, one for each request in\nAsyncBatchAnnotateFilesRequest."]
        #[serde(rename = "responses", default)]
        pub responses: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P4Beta1AsyncAnnotateFileResponse>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1AsyncBatchAnnotateFilesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1AsyncBatchAnnotateImagesResponse {
        #[doc = "The output location and metadata from AsyncBatchAnnotateImagesRequest."]
        #[serde(rename = "outputConfig", default)]
        pub output_config:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P4Beta1OutputConfig>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVisionV1P4Beta1AsyncBatchAnnotateImagesResponse
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVisionV1P4Beta1BatchAnnotateFilesResponse {
        #[doc = "The list of file annotation responses, each response corresponding to each\nAnnotateFileRequest in BatchAnnotateFilesRequest."]
        #[serde(rename = "responses", default)]
        pub responses: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P4Beta1AnnotateFileResponse>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1BatchAnnotateFilesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1BatchOperationMetadata {
        #[doc = "The time when the batch request is finished and\ngoogle.longrunning.Operation.done is set to true."]
        #[serde(rename = "endTime", default)]
        pub end_time: ::std::option::Option<String>,
        #[doc = "The current state of the batch operation."]
        #[serde(rename = "state", default)]
        pub state: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P4Beta1BatchOperationMetadataState,
        >,
        #[doc = "The time when the batch request was submitted to the server."]
        #[serde(rename = "submitTime", default)]
        pub submit_time: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1BatchOperationMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P4Beta1BatchOperationMetadataState {
        #[doc = "The request is done after the longrunning.Operations.CancelOperation has\nbeen called by the user.  Any records that were processed before the\ncancel command are output as specified in the request."]
        Cancelled,
        #[doc = "The request is done and no item has been successfully processed."]
        Failed,
        #[doc = "Request is actively being processed."]
        Processing,
        #[doc = "Invalid."]
        StateUnspecified,
        #[doc = "The request is done and at least one item has been successfully\nprocessed."]
        Successful,
    }
    impl GoogleCloudVisionV1P4Beta1BatchOperationMetadataState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P4Beta1BatchOperationMetadataState::Cancelled => "CANCELLED",
                GoogleCloudVisionV1P4Beta1BatchOperationMetadataState::Failed => "FAILED",
                GoogleCloudVisionV1P4Beta1BatchOperationMetadataState::Processing => "PROCESSING",
                GoogleCloudVisionV1P4Beta1BatchOperationMetadataState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
                GoogleCloudVisionV1P4Beta1BatchOperationMetadataState::Successful => "SUCCESSFUL",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P4Beta1BatchOperationMetadataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P4Beta1BatchOperationMetadataState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P4Beta1BatchOperationMetadataState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => GoogleCloudVisionV1P4Beta1BatchOperationMetadataState::Cancelled,
                "FAILED" => GoogleCloudVisionV1P4Beta1BatchOperationMetadataState::Failed,
                "PROCESSING" => GoogleCloudVisionV1P4Beta1BatchOperationMetadataState::Processing,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudVisionV1P4Beta1BatchOperationMetadataState::StateUnspecified
                }
                "SUCCESSFUL" => GoogleCloudVisionV1P4Beta1BatchOperationMetadataState::Successful,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1BatchOperationMetadataState {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1Block {
        #[doc = "Detected block type (text, image etc) for this block."]
        #[serde(rename = "blockType", default)]
        pub block_type:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P4Beta1BlockBlockType>,
        #[doc = "The bounding box for the block.\nThe vertices are in the order of top-left, top-right, bottom-right,\nbottom-left. When a rotation of the bounding box is detected the rotation\nis represented as around the top-left corner as defined when the text is\nread in the 'natural' orientation.\nFor example:\n\n* when the text is horizontal it might look like:\n  \n  ````text\n    0----1\n    |    |\n    3----2\n  ````\n\n* when it's rotated 180 degrees around the top-left corner it becomes:\n  \n  ````text\n    2----3\n    |    |\n    1----0\n  ````\n  \n  and the vertex order will still be (0, 1, 2, 3)."]
        #[serde(rename = "boundingBox", default)]
        pub bounding_box:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P4Beta1BoundingPoly>,
        #[doc = "Confidence of the OCR results on the block. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "List of paragraphs in this block (if this blocks is of type text)."]
        #[serde(rename = "paragraphs", default)]
        pub paragraphs:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P4Beta1Paragraph>>,
        #[doc = "Additional information detected for the block."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P4Beta1TextAnnotationTextProperty,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1Block {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P4Beta1BlockBlockType {
        #[doc = "Barcode block."]
        Barcode,
        #[doc = "Image block."]
        Picture,
        #[doc = "Horizontal/vertical line box."]
        Ruler,
        #[doc = "Table block."]
        Table,
        #[doc = "Regular text block."]
        Text,
        #[doc = "Unknown block type."]
        Unknown,
    }
    impl GoogleCloudVisionV1P4Beta1BlockBlockType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P4Beta1BlockBlockType::Barcode => "BARCODE",
                GoogleCloudVisionV1P4Beta1BlockBlockType::Picture => "PICTURE",
                GoogleCloudVisionV1P4Beta1BlockBlockType::Ruler => "RULER",
                GoogleCloudVisionV1P4Beta1BlockBlockType::Table => "TABLE",
                GoogleCloudVisionV1P4Beta1BlockBlockType::Text => "TEXT",
                GoogleCloudVisionV1P4Beta1BlockBlockType::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P4Beta1BlockBlockType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P4Beta1BlockBlockType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P4Beta1BlockBlockType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BARCODE" => GoogleCloudVisionV1P4Beta1BlockBlockType::Barcode,
                "PICTURE" => GoogleCloudVisionV1P4Beta1BlockBlockType::Picture,
                "RULER" => GoogleCloudVisionV1P4Beta1BlockBlockType::Ruler,
                "TABLE" => GoogleCloudVisionV1P4Beta1BlockBlockType::Table,
                "TEXT" => GoogleCloudVisionV1P4Beta1BlockBlockType::Text,
                "UNKNOWN" => GoogleCloudVisionV1P4Beta1BlockBlockType::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1BlockBlockType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1BoundingPoly {
        #[doc = "The bounding polygon normalized vertices."]
        #[serde(rename = "normalizedVertices", default)]
        pub normalized_vertices:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P4Beta1NormalizedVertex>>,
        #[doc = "The bounding polygon vertices."]
        #[serde(rename = "vertices", default)]
        pub vertices: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P4Beta1Vertex>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1BoundingPoly {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1ColorInfo {
        #[doc = "RGB components of the color."]
        #[serde(rename = "color", default)]
        pub color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The fraction of pixels the color occupies in the image.\nValue in range [0, 1]."]
        #[serde(rename = "pixelFraction", default)]
        pub pixel_fraction: ::std::option::Option<f32>,
        #[doc = "Image-specific score for this color. Value in range [0, 1]."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1ColorInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1CropHint {
        #[doc = "The bounding polygon for the crop region. The coordinates of the bounding\nbox are in the original image's scale."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P4Beta1BoundingPoly>,
        #[doc = "Confidence of this being a salient region.  Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Fraction of importance of this salient region with respect to the original\nimage."]
        #[serde(rename = "importanceFraction", default)]
        pub importance_fraction: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1CropHint {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1CropHintsAnnotation {
        #[doc = "Crop hint results."]
        #[serde(rename = "cropHints", default)]
        pub crop_hints:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P4Beta1CropHint>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1CropHintsAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1DominantColorsAnnotation {
        #[doc = "RGB color values with their score and pixel fraction."]
        #[serde(rename = "colors", default)]
        pub colors: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P4Beta1ColorInfo>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1DominantColorsAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1EntityAnnotation {
        #[doc = "Image region to which this entity belongs. Not produced\nfor `LABEL_DETECTION` features."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P4Beta1BoundingPoly>,
        #[doc = "**Deprecated. Use `score` instead.**\nThe accuracy of the entity detection in an image.\nFor example, for an image in which the \"Eiffel Tower\" entity is detected,\nthis field represents the confidence that there is a tower in the query\nimage. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Entity textual description, expressed in its `locale` language."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "The language code for the locale in which the entity textual\n`description` is expressed."]
        #[serde(rename = "locale", default)]
        pub locale: ::std::option::Option<String>,
        #[doc = "The location information for the detected entity. Multiple\n`LocationInfo` elements can be present because one location may\nindicate the location of the scene in the image, and another location\nmay indicate the location of the place where the image was taken.\nLocation information is usually present for landmarks."]
        #[serde(rename = "locations", default)]
        pub locations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P4Beta1LocationInfo>>,
        #[doc = "Opaque entity ID. Some IDs may be available in\n[Google Knowledge Graph Search\nAPI](https://developers.google.com/knowledge-graph/)."]
        #[serde(rename = "mid", default)]
        pub mid: ::std::option::Option<String>,
        #[doc = "Some entities may have optional user-supplied `Property` (name/value)\nfields, such a score or string that qualifies the entity."]
        #[serde(rename = "properties", default)]
        pub properties:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P4Beta1Property>>,
        #[doc = "Overall score of the result. Range [0, 1]."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
        #[doc = "The relevancy of the ICA (Image Content Annotation) label to the\nimage. For example, the relevancy of \"tower\" is likely higher to an image\ncontaining the detected \"Eiffel Tower\" than to an image containing a\ndetected distant towering building, even though the confidence that\nthere is a tower in each image may be the same. Range [0, 1]."]
        #[serde(rename = "topicality", default)]
        pub topicality: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1EntityAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1FaceAnnotation {
        #[doc = "Anger likelihood."]
        #[serde(rename = "angerLikelihood", default)]
        pub anger_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P4Beta1FaceAnnotationAngerLikelihood,
        >,
        #[doc = "Blurred likelihood."]
        #[serde(rename = "blurredLikelihood", default)]
        pub blurred_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P4Beta1FaceAnnotationBlurredLikelihood,
        >,
        #[doc = "The bounding polygon around the face. The coordinates of the bounding box\nare in the original image's scale.\nThe bounding box is computed to \"frame\" the face in accordance with human\nexpectations. It is based on the landmarker results.\nNote that one or more x and/or y coordinates may not be generated in the\n`BoundingPoly` (the polygon will be unbounded) if only a partial face\nappears in the image to be annotated."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P4Beta1BoundingPoly>,
        #[doc = "Detection confidence. Range [0, 1]."]
        #[serde(rename = "detectionConfidence", default)]
        pub detection_confidence: ::std::option::Option<f32>,
        #[doc = "The `fd_bounding_poly` bounding polygon is tighter than the\n`boundingPoly`, and encloses only the skin part of the face. Typically, it\nis used to eliminate the face from any image analysis that detects the\n\"amount of skin\" visible in an image. It is not based on the\nlandmarker results, only on the initial face detection, hence\nthe <code>fd</code> (face detection) prefix."]
        #[serde(rename = "fdBoundingPoly", default)]
        pub fd_bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P4Beta1BoundingPoly>,
        #[doc = "Headwear likelihood."]
        #[serde(rename = "headwearLikelihood", default)]
        pub headwear_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P4Beta1FaceAnnotationHeadwearLikelihood,
        >,
        #[doc = "Joy likelihood."]
        #[serde(rename = "joyLikelihood", default)]
        pub joy_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P4Beta1FaceAnnotationJoyLikelihood,
        >,
        #[doc = "Face landmarking confidence. Range [0, 1]."]
        #[serde(rename = "landmarkingConfidence", default)]
        pub landmarking_confidence: ::std::option::Option<f32>,
        #[doc = "Detected face landmarks."]
        #[serde(rename = "landmarks", default)]
        pub landmarks: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P4Beta1FaceAnnotationLandmark>,
        >,
        #[doc = "Yaw angle, which indicates the leftward/rightward angle that the face is\npointing relative to the vertical plane perpendicular to the image. Range\n[-180,180]."]
        #[serde(rename = "panAngle", default)]
        pub pan_angle: ::std::option::Option<f32>,
        #[doc = "Roll angle, which indicates the amount of clockwise/anti-clockwise rotation\nof the face relative to the image vertical about the axis perpendicular to\nthe face. Range [-180,180]."]
        #[serde(rename = "rollAngle", default)]
        pub roll_angle: ::std::option::Option<f32>,
        #[doc = "Sorrow likelihood."]
        #[serde(rename = "sorrowLikelihood", default)]
        pub sorrow_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P4Beta1FaceAnnotationSorrowLikelihood,
        >,
        #[doc = "Surprise likelihood."]
        #[serde(rename = "surpriseLikelihood", default)]
        pub surprise_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P4Beta1FaceAnnotationSurpriseLikelihood,
        >,
        #[doc = "Pitch angle, which indicates the upwards/downwards angle that the face is\npointing relative to the image's horizontal plane. Range [-180,180]."]
        #[serde(rename = "tiltAngle", default)]
        pub tilt_angle: ::std::option::Option<f32>,
        #[doc = "Under-exposed likelihood."]
        #[serde(rename = "underExposedLikelihood", default)]
        pub under_exposed_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P4Beta1FaceAnnotationUnderExposedLikelihood,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1FaceAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P4Beta1FaceAnnotationAngerLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P4Beta1FaceAnnotationAngerLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P4Beta1FaceAnnotationAngerLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P4Beta1FaceAnnotationAngerLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P4Beta1FaceAnnotationAngerLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P4Beta1FaceAnnotationAngerLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P4Beta1FaceAnnotationAngerLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationAngerLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P4Beta1FaceAnnotationAngerLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P4Beta1FaceAnnotationAngerLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P4Beta1FaceAnnotationAngerLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P4Beta1FaceAnnotationAngerLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P4Beta1FaceAnnotationAngerLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P4Beta1FaceAnnotationAngerLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P4Beta1FaceAnnotationAngerLikelihood::Unlikely,
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationAngerLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationAngerLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1FaceAnnotationAngerLikelihood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P4Beta1FaceAnnotationBlurredLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P4Beta1FaceAnnotationBlurredLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P4Beta1FaceAnnotationBlurredLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P4Beta1FaceAnnotationBlurredLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P4Beta1FaceAnnotationBlurredLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P4Beta1FaceAnnotationBlurredLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P4Beta1FaceAnnotationBlurredLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationBlurredLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P4Beta1FaceAnnotationBlurredLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P4Beta1FaceAnnotationBlurredLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P4Beta1FaceAnnotationBlurredLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P4Beta1FaceAnnotationBlurredLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P4Beta1FaceAnnotationBlurredLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P4Beta1FaceAnnotationBlurredLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P4Beta1FaceAnnotationBlurredLikelihood::Unlikely,
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationBlurredLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationBlurredLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1FaceAnnotationBlurredLikelihood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P4Beta1FaceAnnotationHeadwearLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P4Beta1FaceAnnotationHeadwearLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P4Beta1FaceAnnotationHeadwearLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P4Beta1FaceAnnotationHeadwearLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P4Beta1FaceAnnotationHeadwearLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P4Beta1FaceAnnotationHeadwearLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P4Beta1FaceAnnotationHeadwearLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationHeadwearLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P4Beta1FaceAnnotationHeadwearLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P4Beta1FaceAnnotationHeadwearLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P4Beta1FaceAnnotationHeadwearLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P4Beta1FaceAnnotationHeadwearLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P4Beta1FaceAnnotationHeadwearLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P4Beta1FaceAnnotationHeadwearLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P4Beta1FaceAnnotationHeadwearLikelihood::Unlikely,
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationHeadwearLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationHeadwearLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVisionV1P4Beta1FaceAnnotationHeadwearLikelihood
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P4Beta1FaceAnnotationJoyLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P4Beta1FaceAnnotationJoyLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P4Beta1FaceAnnotationJoyLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P4Beta1FaceAnnotationJoyLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P4Beta1FaceAnnotationJoyLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P4Beta1FaceAnnotationJoyLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P4Beta1FaceAnnotationJoyLikelihood::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P4Beta1FaceAnnotationJoyLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P4Beta1FaceAnnotationJoyLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P4Beta1FaceAnnotationJoyLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P4Beta1FaceAnnotationJoyLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P4Beta1FaceAnnotationJoyLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P4Beta1FaceAnnotationJoyLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P4Beta1FaceAnnotationJoyLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P4Beta1FaceAnnotationJoyLikelihood::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P4Beta1FaceAnnotationJoyLikelihood::VeryLikely,
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationJoyLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1FaceAnnotationJoyLikelihood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P4Beta1FaceAnnotationSorrowLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P4Beta1FaceAnnotationSorrowLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P4Beta1FaceAnnotationSorrowLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P4Beta1FaceAnnotationSorrowLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P4Beta1FaceAnnotationSorrowLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P4Beta1FaceAnnotationSorrowLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P4Beta1FaceAnnotationSorrowLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationSorrowLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P4Beta1FaceAnnotationSorrowLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P4Beta1FaceAnnotationSorrowLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P4Beta1FaceAnnotationSorrowLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P4Beta1FaceAnnotationSorrowLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P4Beta1FaceAnnotationSorrowLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P4Beta1FaceAnnotationSorrowLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P4Beta1FaceAnnotationSorrowLikelihood::Unlikely,
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationSorrowLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationSorrowLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1FaceAnnotationSorrowLikelihood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P4Beta1FaceAnnotationSurpriseLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P4Beta1FaceAnnotationSurpriseLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P4Beta1FaceAnnotationSurpriseLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P4Beta1FaceAnnotationSurpriseLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P4Beta1FaceAnnotationSurpriseLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P4Beta1FaceAnnotationSurpriseLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P4Beta1FaceAnnotationSurpriseLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationSurpriseLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P4Beta1FaceAnnotationSurpriseLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P4Beta1FaceAnnotationSurpriseLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P4Beta1FaceAnnotationSurpriseLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P4Beta1FaceAnnotationSurpriseLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P4Beta1FaceAnnotationSurpriseLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P4Beta1FaceAnnotationSurpriseLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P4Beta1FaceAnnotationSurpriseLikelihood::Unlikely,
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationSurpriseLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationSurpriseLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVisionV1P4Beta1FaceAnnotationSurpriseLikelihood
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P4Beta1FaceAnnotationUnderExposedLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P4Beta1FaceAnnotationUnderExposedLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P4Beta1FaceAnnotationUnderExposedLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P4Beta1FaceAnnotationUnderExposedLikelihood::Possible => {
                    "POSSIBLE"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationUnderExposedLikelihood::Unknown => {
                    "UNKNOWN"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationUnderExposedLikelihood::Unlikely => {
                    "UNLIKELY"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationUnderExposedLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationUnderExposedLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P4Beta1FaceAnnotationUnderExposedLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P4Beta1FaceAnnotationUnderExposedLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVisionV1P4Beta1FaceAnnotationUnderExposedLikelihood
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P4Beta1FaceAnnotationUnderExposedLikelihood::Likely,
                "POSSIBLE" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationUnderExposedLikelihood::Possible
                }
                "UNKNOWN" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationUnderExposedLikelihood::Unknown
                }
                "UNLIKELY" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationUnderExposedLikelihood::Unlikely
                }
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationUnderExposedLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationUnderExposedLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVisionV1P4Beta1FaceAnnotationUnderExposedLikelihood
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1FaceAnnotationLandmark {
        #[doc = "Face landmark position."]
        #[serde(rename = "position", default)]
        pub position: ::std::option::Option<crate::schemas::GoogleCloudVisionV1P4Beta1Position>,
        #[doc = "Face landmark type."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1FaceAnnotationLandmark {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType {
        #[doc = "Chin gnathion."]
        ChinGnathion,
        #[doc = "Chin left gonion."]
        ChinLeftGonion,
        #[doc = "Chin right gonion."]
        ChinRightGonion,
        #[doc = "Forehead glabella."]
        ForeheadGlabella,
        #[doc = "Left ear tragion."]
        LeftEarTragion,
        #[doc = "Left eye."]
        LeftEye,
        #[doc = "Left eye, bottom boundary."]
        LeftEyeBottomBoundary,
        #[doc = "Left eye, left corner."]
        LeftEyeLeftCorner,
        #[doc = "Left eye pupil."]
        LeftEyePupil,
        #[doc = "Left eye, right corner."]
        LeftEyeRightCorner,
        #[doc = "Left eye, top boundary."]
        LeftEyeTopBoundary,
        #[doc = "Left eyebrow, upper midpoint."]
        LeftEyebrowUpperMidpoint,
        #[doc = "Left of left eyebrow."]
        LeftOfLeftEyebrow,
        #[doc = "Left of right eyebrow."]
        LeftOfRightEyebrow,
        #[doc = "Lower lip."]
        LowerLip,
        #[doc = "Midpoint between eyes."]
        MidpointBetweenEyes,
        #[doc = "Mouth center."]
        MouthCenter,
        #[doc = "Mouth left."]
        MouthLeft,
        #[doc = "Mouth right."]
        MouthRight,
        #[doc = "Nose, bottom center."]
        NoseBottomCenter,
        #[doc = "Nose, bottom left."]
        NoseBottomLeft,
        #[doc = "Nose, bottom right."]
        NoseBottomRight,
        #[doc = "Nose tip."]
        NoseTip,
        #[doc = "Right ear tragion."]
        RightEarTragion,
        #[doc = "Right eye."]
        RightEye,
        #[doc = "Right eye, bottom boundary."]
        RightEyeBottomBoundary,
        #[doc = "Right eye, left corner."]
        RightEyeLeftCorner,
        #[doc = "Right eye pupil."]
        RightEyePupil,
        #[doc = "Right eye, right corner."]
        RightEyeRightCorner,
        #[doc = "Right eye, top boundary."]
        RightEyeTopBoundary,
        #[doc = "Right eyebrow, upper midpoint."]
        RightEyebrowUpperMidpoint,
        #[doc = "Right of left eyebrow."]
        RightOfLeftEyebrow,
        #[doc = "Right of right eyebrow."]
        RightOfRightEyebrow,
        #[doc = "Unknown face landmark detected. Should not be filled."]
        UnknownLandmark,
        #[doc = "Upper lip."]
        UpperLip,
    }
    impl GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::ChinGnathion => {
                    "CHIN_GNATHION"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::ChinLeftGonion => {
                    "CHIN_LEFT_GONION"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::ChinRightGonion => {
                    "CHIN_RIGHT_GONION"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::ForeheadGlabella => {
                    "FOREHEAD_GLABELLA"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::LeftEarTragion => {
                    "LEFT_EAR_TRAGION"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::LeftEye => "LEFT_EYE",
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::LeftEyeBottomBoundary => {
                    "LEFT_EYE_BOTTOM_BOUNDARY"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::LeftEyeLeftCorner => {
                    "LEFT_EYE_LEFT_CORNER"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::LeftEyePupil => {
                    "LEFT_EYE_PUPIL"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::LeftEyeRightCorner => {
                    "LEFT_EYE_RIGHT_CORNER"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::LeftEyeTopBoundary => {
                    "LEFT_EYE_TOP_BOUNDARY"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::LeftEyebrowUpperMidpoint => {
                    "LEFT_EYEBROW_UPPER_MIDPOINT"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::LeftOfLeftEyebrow => {
                    "LEFT_OF_LEFT_EYEBROW"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::LeftOfRightEyebrow => {
                    "LEFT_OF_RIGHT_EYEBROW"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::LowerLip => "LOWER_LIP",
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::MidpointBetweenEyes => {
                    "MIDPOINT_BETWEEN_EYES"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::MouthCenter => "MOUTH_CENTER",
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::MouthLeft => "MOUTH_LEFT",
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::MouthRight => "MOUTH_RIGHT",
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::NoseBottomCenter => {
                    "NOSE_BOTTOM_CENTER"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::NoseBottomLeft => {
                    "NOSE_BOTTOM_LEFT"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::NoseBottomRight => {
                    "NOSE_BOTTOM_RIGHT"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::NoseTip => "NOSE_TIP",
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::RightEarTragion => {
                    "RIGHT_EAR_TRAGION"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::RightEye => "RIGHT_EYE",
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::RightEyeBottomBoundary => {
                    "RIGHT_EYE_BOTTOM_BOUNDARY"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::RightEyeLeftCorner => {
                    "RIGHT_EYE_LEFT_CORNER"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::RightEyePupil => {
                    "RIGHT_EYE_PUPIL"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::RightEyeRightCorner => {
                    "RIGHT_EYE_RIGHT_CORNER"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::RightEyeTopBoundary => {
                    "RIGHT_EYE_TOP_BOUNDARY"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::RightEyebrowUpperMidpoint => {
                    "RIGHT_EYEBROW_UPPER_MIDPOINT"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::RightOfLeftEyebrow => {
                    "RIGHT_OF_LEFT_EYEBROW"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::RightOfRightEyebrow => {
                    "RIGHT_OF_RIGHT_EYEBROW"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::UnknownLandmark => {
                    "UNKNOWN_LANDMARK"
                }
                GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::UpperLip => "UPPER_LIP",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CHIN_GNATHION" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::ChinGnathion
                }
                "CHIN_LEFT_GONION" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::ChinLeftGonion
                }
                "CHIN_RIGHT_GONION" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::ChinRightGonion
                }
                "FOREHEAD_GLABELLA" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::ForeheadGlabella
                }
                "LEFT_EAR_TRAGION" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::LeftEarTragion
                }
                "LEFT_EYE" => GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::LeftEye,
                "LEFT_EYE_BOTTOM_BOUNDARY" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::LeftEyeBottomBoundary
                }
                "LEFT_EYE_LEFT_CORNER" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::LeftEyeLeftCorner
                }
                "LEFT_EYE_PUPIL" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::LeftEyePupil
                }
                "LEFT_EYE_RIGHT_CORNER" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::LeftEyeRightCorner
                }
                "LEFT_EYE_TOP_BOUNDARY" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::LeftEyeTopBoundary
                }
                "LEFT_EYEBROW_UPPER_MIDPOINT" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::LeftEyebrowUpperMidpoint
                }
                "LEFT_OF_LEFT_EYEBROW" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::LeftOfLeftEyebrow
                }
                "LEFT_OF_RIGHT_EYEBROW" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::LeftOfRightEyebrow
                }
                "LOWER_LIP" => GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::LowerLip,
                "MIDPOINT_BETWEEN_EYES" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::MidpointBetweenEyes
                }
                "MOUTH_CENTER" => GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::MouthCenter,
                "MOUTH_LEFT" => GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::MouthLeft,
                "MOUTH_RIGHT" => GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::MouthRight,
                "NOSE_BOTTOM_CENTER" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::NoseBottomCenter
                }
                "NOSE_BOTTOM_LEFT" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::NoseBottomLeft
                }
                "NOSE_BOTTOM_RIGHT" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::NoseBottomRight
                }
                "NOSE_TIP" => GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::NoseTip,
                "RIGHT_EAR_TRAGION" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::RightEarTragion
                }
                "RIGHT_EYE" => GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::RightEye,
                "RIGHT_EYE_BOTTOM_BOUNDARY" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::RightEyeBottomBoundary
                }
                "RIGHT_EYE_LEFT_CORNER" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::RightEyeLeftCorner
                }
                "RIGHT_EYE_PUPIL" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::RightEyePupil
                }
                "RIGHT_EYE_RIGHT_CORNER" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::RightEyeRightCorner
                }
                "RIGHT_EYE_TOP_BOUNDARY" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::RightEyeTopBoundary
                }
                "RIGHT_EYEBROW_UPPER_MIDPOINT" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::RightEyebrowUpperMidpoint
                }
                "RIGHT_OF_LEFT_EYEBROW" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::RightOfLeftEyebrow
                }
                "RIGHT_OF_RIGHT_EYEBROW" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::RightOfRightEyebrow
                }
                "UNKNOWN_LANDMARK" => {
                    GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::UnknownLandmark
                }
                "UPPER_LIP" => GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType::UpperLip,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1FaceAnnotationLandmarkType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1GcsDestination {
        #[doc = "Google Cloud Storage URI prefix where the results will be stored. Results\nwill be in JSON format and preceded by its corresponding input URI prefix.\nThis field can either represent a gcs file prefix or gcs directory. In\neither case, the uri should be unique because in order to get all of the\noutput files, you will need to do a wildcard gcs search on the uri prefix\nyou provide.\n\nExamples:\n\n* File Prefix: gs://bucket-name/here/filenameprefix   The output files\n  will be created in gs://bucket-name/here/ and the names of the\n  output files will begin with \"filenameprefix\".\n\n* Directory Prefix: gs://bucket-name/some/location/   The output files\n  will be created in gs://bucket-name/some/location/ and the names of the\n  output files could be anything because there was no filename prefix\n  specified.\n\nIf multiple outputs, each response is still AnnotateFileResponse, each of\nwhich contains some subset of the full list of AnnotateImageResponse.\nMultiple outputs can happen if, for example, the output JSON is too large\nand overflows into multiple sharded files."]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1GcsDestination {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1GcsSource {
        #[doc = "Google Cloud Storage URI for the input file. This must only be a\nGoogle Cloud Storage object. Wildcards are not currently supported."]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1GcsSource {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1ImageAnnotationContext {
        #[doc = "If the file was a PDF or TIFF, this field gives the page number within\nthe file used to produce the image."]
        #[serde(rename = "pageNumber", default)]
        pub page_number: ::std::option::Option<i32>,
        #[doc = "The URI of the file used to produce the image."]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1ImageAnnotationContext {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1ImageProperties {
        #[doc = "If present, dominant colors completed successfully."]
        #[serde(rename = "dominantColors", default)]
        pub dominant_colors: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P4Beta1DominantColorsAnnotation,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1ImageProperties {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1ImageQuality {
        #[doc = "A score representing the aesthetic/technical quality of the image. The\nscore is in range [0, 1]. Higher value corresponds to more professional\nlooking photos. 0 means the image looks very bad, 1 means the image with\nvery high quality."]
        #[serde(rename = "qualityScore", default)]
        pub quality_score: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1ImageQuality {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVisionV1P4Beta1ImportProductSetsResponse {
        #[doc = "The list of reference_images that are imported successfully."]
        #[serde(rename = "referenceImages", default)]
        pub reference_images:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P4Beta1ReferenceImage>>,
        #[doc = "The rpc status for each ImportProductSet request, including both successes\nand errors.\n\nThe number of statuses here matches the number of lines in the csv file,\nand statuses[i] stores the success or failure status of processing the i-th\nline of the csv, starting from line 0."]
        #[serde(rename = "statuses", default)]
        pub statuses: ::std::option::Option<Vec<crate::schemas::Status>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1ImportProductSetsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1InputConfig {
        #[doc = "File content, represented as a stream of bytes.\nNote: As with all `bytes` fields, protobuffers use a pure binary\nrepresentation, whereas JSON representations use base64.\n\nCurrently, this field only works for BatchAnnotateFiles requests. It does\nnot work for AsyncBatchAnnotateFiles requests."]
        #[serde(rename = "content", default)]
        pub content: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "The Google Cloud Storage location to read the input from."]
        #[serde(rename = "gcsSource", default)]
        pub gcs_source: ::std::option::Option<crate::schemas::GoogleCloudVisionV1P4Beta1GcsSource>,
        #[doc = "The type of the file. Currently only \"application/pdf\", \"image/tiff\" and\n\"image/gif\" are supported. Wildcards are not supported."]
        #[serde(rename = "mimeType", default)]
        pub mime_type: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1InputConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1LocalizedObjectAnnotation {
        #[doc = "Image region to which this object belongs. This must be populated."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P4Beta1BoundingPoly>,
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more\ninformation, see\nhttp://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Object ID that should align with EntityAnnotation mid."]
        #[serde(rename = "mid", default)]
        pub mid: ::std::option::Option<String>,
        #[doc = "Object name, expressed in its `language_code` language."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Score of the result. Range [0, 1]."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1LocalizedObjectAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1LocationInfo {
        #[doc = "lat/long location coordinates."]
        #[serde(rename = "latLng", default)]
        pub lat_lng: ::std::option::Option<crate::schemas::LatLng>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1LocationInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1NormalizedVertex {
        #[doc = "X coordinate."]
        #[serde(rename = "x", default)]
        pub x: ::std::option::Option<f32>,
        #[doc = "Y coordinate."]
        #[serde(rename = "y", default)]
        pub y: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1NormalizedVertex {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1OperationMetadata {
        #[doc = "The time when the batch request was received."]
        #[serde(rename = "createTime", default)]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Current state of the batch operation."]
        #[serde(rename = "state", default)]
        pub state:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P4Beta1OperationMetadataState>,
        #[doc = "The time when the operation result was last updated."]
        #[serde(rename = "updateTime", default)]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1OperationMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P4Beta1OperationMetadataState {
        #[doc = "The batch processing was cancelled."]
        Cancelled,
        #[doc = "Request is received."]
        Created,
        #[doc = "The batch processing is done."]
        Done,
        #[doc = "Request is actively being processed."]
        Running,
        #[doc = "Invalid."]
        StateUnspecified,
    }
    impl GoogleCloudVisionV1P4Beta1OperationMetadataState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P4Beta1OperationMetadataState::Cancelled => "CANCELLED",
                GoogleCloudVisionV1P4Beta1OperationMetadataState::Created => "CREATED",
                GoogleCloudVisionV1P4Beta1OperationMetadataState::Done => "DONE",
                GoogleCloudVisionV1P4Beta1OperationMetadataState::Running => "RUNNING",
                GoogleCloudVisionV1P4Beta1OperationMetadataState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P4Beta1OperationMetadataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P4Beta1OperationMetadataState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P4Beta1OperationMetadataState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => GoogleCloudVisionV1P4Beta1OperationMetadataState::Cancelled,
                "CREATED" => GoogleCloudVisionV1P4Beta1OperationMetadataState::Created,
                "DONE" => GoogleCloudVisionV1P4Beta1OperationMetadataState::Done,
                "RUNNING" => GoogleCloudVisionV1P4Beta1OperationMetadataState::Running,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudVisionV1P4Beta1OperationMetadataState::StateUnspecified
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1OperationMetadataState {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1OutputConfig {
        #[doc = "The max number of response protos to put into each output JSON file on\nGoogle Cloud Storage.\nThe valid range is [1, 100]. If not specified, the default value is 20.\n\nFor example, for one pdf file with 100 pages, 100 response protos will\nbe generated. If `batch_size` = 20, then 5 json files each\ncontaining 20 response protos will be written under the prefix\n`gcs_destination`.`uri`.\n\nCurrently, batch_size only applies to GcsDestination, with potential future\nsupport for other output configurations."]
        #[serde(rename = "batchSize", default)]
        pub batch_size: ::std::option::Option<i32>,
        #[doc = "The Google Cloud Storage location to write the output(s) to."]
        #[serde(rename = "gcsDestination", default)]
        pub gcs_destination:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P4Beta1GcsDestination>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1OutputConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1Page {
        #[doc = "List of blocks of text, images etc on this page."]
        #[serde(rename = "blocks", default)]
        pub blocks: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P4Beta1Block>>,
        #[doc = "Confidence of the OCR results on the page. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Page height. For PDFs the unit is points. For images (including\nTIFFs) the unit is pixels."]
        #[serde(rename = "height", default)]
        pub height: ::std::option::Option<i32>,
        #[doc = "Additional information detected on the page."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P4Beta1TextAnnotationTextProperty,
        >,
        #[doc = "Page width. For PDFs the unit is points. For images (including\nTIFFs) the unit is pixels."]
        #[serde(rename = "width", default)]
        pub width: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1Page {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1Paragraph {
        #[doc = "The bounding box for the paragraph.\nThe vertices are in the order of top-left, top-right, bottom-right,\nbottom-left. When a rotation of the bounding box is detected the rotation\nis represented as around the top-left corner as defined when the text is\nread in the 'natural' orientation.\nFor example:\n\n* when the text is horizontal it might look like:\n  0----1\n  |    |\n  3----2\n* when it's rotated 180 degrees around the top-left corner it becomes:\n  2----3\n  |    |\n  1----0\n  and the vertex order will still be (0, 1, 2, 3)."]
        #[serde(rename = "boundingBox", default)]
        pub bounding_box:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P4Beta1BoundingPoly>,
        #[doc = "Confidence of the OCR results for the paragraph. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Additional information detected for the paragraph."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P4Beta1TextAnnotationTextProperty,
        >,
        #[doc = "List of all words in this paragraph."]
        #[serde(rename = "words", default)]
        pub words: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P4Beta1Word>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1Paragraph {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1Position {
        #[doc = "X coordinate."]
        #[serde(rename = "x", default)]
        pub x: ::std::option::Option<f32>,
        #[doc = "Y coordinate."]
        #[serde(rename = "y", default)]
        pub y: ::std::option::Option<f32>,
        #[doc = "Z coordinate (or depth)."]
        #[serde(rename = "z", default)]
        pub z: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1Position {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1Product {
        #[doc = "User-provided metadata to be stored with this product. Must be at most 4096\ncharacters long."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "The user-provided name for this Product. Must not be empty. Must be at most\n4096 characters long."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The resource name of the product.\n\nFormat is:\n`projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`.\n\nThis field is ignored when creating a product."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The category for the product identified by the reference image. This should\nbe either \"homegoods-v2\", \"apparel-v2\", or \"toys-v2\". The legacy categories\n\"homegoods\", \"apparel\", and \"toys\" are still supported, but these should\nnot be used for new products.\n\nThis field is immutable."]
        #[serde(rename = "productCategory", default)]
        pub product_category: ::std::option::Option<String>,
        #[doc = "Key-value pairs that can be attached to a product. At query time,\nconstraints can be specified based on the product_labels.\n\nNote that integer values can be provided as strings, e.g. \"1199\". Only\nstrings with integer values can match a range-based restriction which is\nto be supported soon.\n\nMultiple values can be assigned to the same key. One product may have up to\n500 product_labels.\n\nNotice that the total number of distinct product_labels over all products\nin one ProductSet cannot exceed 1M, otherwise the product search pipeline\nwill refuse to work for that ProductSet."]
        #[serde(rename = "productLabels", default)]
        pub product_labels:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P4Beta1ProductKeyValue>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1Product {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1ProductKeyValue {
        #[doc = "The key of the label attached to the product. Cannot be empty and cannot\nexceed 128 bytes."]
        #[serde(rename = "key", default)]
        pub key: ::std::option::Option<String>,
        #[doc = "The value of the label attached to the product. Cannot be empty and\ncannot exceed 128 bytes."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1ProductKeyValue {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1ProductSearchResults {
        #[doc = "Timestamp of the index which provided these results. Products added to the\nproduct set and products removed from the product set after this time are\nnot reflected in the current results."]
        #[serde(rename = "indexTime", default)]
        pub index_time: ::std::option::Option<String>,
        #[doc = "List of results grouped by products detected in the query image. Each entry\ncorresponds to one bounding polygon in the query image, and contains the\nmatching products specific to that region. There may be duplicate product\nmatches in the union of all the per-product results."]
        #[serde(rename = "productGroupedResults", default)]
        pub product_grouped_results: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P4Beta1ProductSearchResultsGroupedResult>,
        >,
        #[doc = "List of results, one for each product match."]
        #[serde(rename = "results", default)]
        pub results: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P4Beta1ProductSearchResultsResult>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1ProductSearchResults {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1ProductSearchResultsGroupedResult {
        #[doc = "The bounding polygon around the product detected in the query image."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P4Beta1BoundingPoly>,
        #[doc = "List of results, one for each product match."]
        #[serde(rename = "results", default)]
        pub results: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P4Beta1ProductSearchResultsResult>,
        >,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVisionV1P4Beta1ProductSearchResultsGroupedResult
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1ProductSearchResultsResult {
        #[doc = "The resource name of the image from the product that is the closest match\nto the query."]
        #[serde(rename = "image", default)]
        pub image: ::std::option::Option<String>,
        #[doc = "The Product."]
        #[serde(rename = "product", default)]
        pub product: ::std::option::Option<crate::schemas::GoogleCloudVisionV1P4Beta1Product>,
        #[doc = "A confidence level on the match, ranging from 0 (no confidence) to\n1 (full confidence)."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1ProductSearchResultsResult {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1Property {
        #[doc = "Name of the property."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Value of numeric properties."]
        #[serde(rename = "uint64Value", default)]
        #[serde(with = "crate::parsed_string")]
        pub uint_64_value: ::std::option::Option<u64>,
        #[doc = "Value of the property."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1Property {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1QualityOptimizationResult { # [ doc = "Optimized image bytes." ] # [ serde ( rename = "image" , default ) ] pub image : :: std :: option :: Option < crate :: bytes :: Bytes > , # [ doc = "Mime type of the output image." ] # [ serde ( rename = "mimeType" , default ) ] pub mime_type : :: std :: option :: Option < String > , # [ doc = "Required optimization type." ] # [ serde ( rename = "qualityOptimizationType" , default ) ] pub quality_optimization_type : :: std :: option :: Option < crate :: schemas :: GoogleCloudVisionV1P4Beta1QualityOptimizationResultQualityOptimizationType > , }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1QualityOptimizationResult {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P4Beta1QualityOptimizationResultQualityOptimizationType {
        #[doc = "Reduce image file size. Detailed params specified in CompressionConfig.\nIf customer do not specify CompressionConfig, it will reduce image file\nsize while not reducing image quality. If customer specify\nCompressionConfig, we will reduce file size while keeping\nCompressionParams.target_quality."]
        Compression,
        #[doc = "Denoise, sharpening, HDR and upscaling. Detailed params specified in\nEnhancementConfig. If customer do not specify EnhancmentConfig, it will\ndo image enhancement using default values. If upscale_ratio not\nspecified, the output image will have the same resolution as input image."]
        Enhancement,
        #[doc = "Query quality score for an image. Detailed params specified in\nQualityScoreConfig. If customer does not specify QualityScoreConfig,\naesthetic score of image will be returned."]
        QualityScore,
        #[doc = "Invalid. Customer must select one Type."]
        TypeUnspecified,
    }
    impl GoogleCloudVisionV1P4Beta1QualityOptimizationResultQualityOptimizationType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudVisionV1P4Beta1QualityOptimizationResultQualityOptimizationType :: Compression => "COMPRESSION" , GoogleCloudVisionV1P4Beta1QualityOptimizationResultQualityOptimizationType :: Enhancement => "ENHANCEMENT" , GoogleCloudVisionV1P4Beta1QualityOptimizationResultQualityOptimizationType :: QualityScore => "QUALITY_SCORE" , GoogleCloudVisionV1P4Beta1QualityOptimizationResultQualityOptimizationType :: TypeUnspecified => "TYPE_UNSPECIFIED" , }
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudVisionV1P4Beta1QualityOptimizationResultQualityOptimizationType
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudVisionV1P4Beta1QualityOptimizationResultQualityOptimizationType
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVisionV1P4Beta1QualityOptimizationResultQualityOptimizationType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "COMPRESSION" => GoogleCloudVisionV1P4Beta1QualityOptimizationResultQualityOptimizationType :: Compression , "ENHANCEMENT" => GoogleCloudVisionV1P4Beta1QualityOptimizationResultQualityOptimizationType :: Enhancement , "QUALITY_SCORE" => GoogleCloudVisionV1P4Beta1QualityOptimizationResultQualityOptimizationType :: QualityScore , "TYPE_UNSPECIFIED" => GoogleCloudVisionV1P4Beta1QualityOptimizationResultQualityOptimizationType :: TypeUnspecified , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVisionV1P4Beta1QualityOptimizationResultQualityOptimizationType
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1ReferenceImage {
        #[doc = "Bounding polygons around the areas of interest in the reference image.\nOptional. If this field is empty, the system will try to detect regions of\ninterest. At most 10 bounding polygons will be used.\n\nThe provided shape is converted into a non-rotated rectangle. Once\nconverted, the small edge of the rectangle must be greater than or equal\nto 300 pixels. The aspect ratio must be 1:4 or less (i.e. 1:3 is ok; 1:5\nis not)."]
        #[serde(rename = "boundingPolys", default)]
        pub bounding_polys:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P4Beta1BoundingPoly>>,
        #[doc = "The resource name of the reference image.\n\nFormat is:\n\n`projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID/referenceImages/IMAGE_ID`.\n\nThis field is ignored when creating a reference image."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The Google Cloud Storage URI of the reference image.\n\nThe URI must start with `gs://`.\n\nRequired."]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1ReferenceImage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1SafeSearchAnnotation {
        #[doc = "Represents the adult content likelihood for the image. Adult content may\ncontain elements such as nudity, pornographic images or cartoons, or\nsexual activities."]
        #[serde(rename = "adult", default)]
        pub adult: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P4Beta1SafeSearchAnnotationAdult,
        >,
        #[doc = "Confidence of adult_score. Range [0, 1]. 0 means not confident, 1 means\nvery confident."]
        #[serde(rename = "adultConfidence", default)]
        pub adult_confidence: ::std::option::Option<f32>,
        #[doc = "Likelihood that this is a medical image."]
        #[serde(rename = "medical", default)]
        pub medical: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P4Beta1SafeSearchAnnotationMedical,
        >,
        #[doc = "Confidence of medical_score. Range [0, 1]. 0 means not confident, 1 means\nvery confident."]
        #[serde(rename = "medicalConfidence", default)]
        pub medical_confidence: ::std::option::Option<f32>,
        #[doc = "Confidence of nsfw_score. Range [0, 1]. 0 means not confident, 1 means very\nconfident."]
        #[serde(rename = "nsfwConfidence", default)]
        pub nsfw_confidence: ::std::option::Option<f32>,
        #[doc = "Likelihood that the request image contains racy content. Racy content may\ninclude (but is not limited to) skimpy or sheer clothing, strategically\ncovered nudity, lewd or provocative poses, or close-ups of sensitive\nbody areas."]
        #[serde(rename = "racy", default)]
        pub racy: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P4Beta1SafeSearchAnnotationRacy,
        >,
        #[doc = "Confidence of racy_score. Range [0, 1]. 0 means not confident, 1 means very\nconfident."]
        #[serde(rename = "racyConfidence", default)]
        pub racy_confidence: ::std::option::Option<f32>,
        #[doc = "Spoof likelihood. The likelihood that an modification\nwas made to the image's canonical version to make it appear\nfunny or offensive."]
        #[serde(rename = "spoof", default)]
        pub spoof: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P4Beta1SafeSearchAnnotationSpoof,
        >,
        #[doc = "Confidence of spoof_score. Range [0, 1]. 0 means not confident, 1 means\nvery confident."]
        #[serde(rename = "spoofConfidence", default)]
        pub spoof_confidence: ::std::option::Option<f32>,
        #[doc = "Likelihood that this image contains violent content."]
        #[serde(rename = "violence", default)]
        pub violence: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P4Beta1SafeSearchAnnotationViolence,
        >,
        #[doc = "Confidence of violence_score. Range [0, 1]. 0 means not confident, 1 means\nvery confident."]
        #[serde(rename = "violenceConfidence", default)]
        pub violence_confidence: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1SafeSearchAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P4Beta1SafeSearchAnnotationAdult {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P4Beta1SafeSearchAnnotationAdult {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationAdult::Likely => "LIKELY",
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationAdult::Possible => "POSSIBLE",
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationAdult::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationAdult::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationAdult::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationAdult::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P4Beta1SafeSearchAnnotationAdult {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P4Beta1SafeSearchAnnotationAdult {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P4Beta1SafeSearchAnnotationAdult {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P4Beta1SafeSearchAnnotationAdult::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P4Beta1SafeSearchAnnotationAdult::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P4Beta1SafeSearchAnnotationAdult::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P4Beta1SafeSearchAnnotationAdult::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P4Beta1SafeSearchAnnotationAdult::VeryLikely,
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P4Beta1SafeSearchAnnotationAdult::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1SafeSearchAnnotationAdult {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P4Beta1SafeSearchAnnotationMedical {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P4Beta1SafeSearchAnnotationMedical {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationMedical::Likely => "LIKELY",
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationMedical::Possible => "POSSIBLE",
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationMedical::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationMedical::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationMedical::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationMedical::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P4Beta1SafeSearchAnnotationMedical {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P4Beta1SafeSearchAnnotationMedical {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P4Beta1SafeSearchAnnotationMedical {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P4Beta1SafeSearchAnnotationMedical::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P4Beta1SafeSearchAnnotationMedical::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P4Beta1SafeSearchAnnotationMedical::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P4Beta1SafeSearchAnnotationMedical::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P4Beta1SafeSearchAnnotationMedical::VeryLikely,
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P4Beta1SafeSearchAnnotationMedical::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1SafeSearchAnnotationMedical {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P4Beta1SafeSearchAnnotationRacy {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P4Beta1SafeSearchAnnotationRacy {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationRacy::Likely => "LIKELY",
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationRacy::Possible => "POSSIBLE",
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationRacy::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationRacy::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationRacy::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationRacy::VeryUnlikely => "VERY_UNLIKELY",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P4Beta1SafeSearchAnnotationRacy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P4Beta1SafeSearchAnnotationRacy {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P4Beta1SafeSearchAnnotationRacy {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P4Beta1SafeSearchAnnotationRacy::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P4Beta1SafeSearchAnnotationRacy::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P4Beta1SafeSearchAnnotationRacy::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P4Beta1SafeSearchAnnotationRacy::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P4Beta1SafeSearchAnnotationRacy::VeryLikely,
                "VERY_UNLIKELY" => GoogleCloudVisionV1P4Beta1SafeSearchAnnotationRacy::VeryUnlikely,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1SafeSearchAnnotationRacy {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P4Beta1SafeSearchAnnotationSpoof {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P4Beta1SafeSearchAnnotationSpoof {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationSpoof::Likely => "LIKELY",
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationSpoof::Possible => "POSSIBLE",
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationSpoof::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationSpoof::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationSpoof::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationSpoof::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P4Beta1SafeSearchAnnotationSpoof {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P4Beta1SafeSearchAnnotationSpoof {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P4Beta1SafeSearchAnnotationSpoof {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P4Beta1SafeSearchAnnotationSpoof::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P4Beta1SafeSearchAnnotationSpoof::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P4Beta1SafeSearchAnnotationSpoof::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P4Beta1SafeSearchAnnotationSpoof::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P4Beta1SafeSearchAnnotationSpoof::VeryLikely,
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P4Beta1SafeSearchAnnotationSpoof::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1SafeSearchAnnotationSpoof {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P4Beta1SafeSearchAnnotationViolence {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P4Beta1SafeSearchAnnotationViolence {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationViolence::Likely => "LIKELY",
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationViolence::Possible => "POSSIBLE",
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationViolence::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationViolence::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationViolence::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P4Beta1SafeSearchAnnotationViolence::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P4Beta1SafeSearchAnnotationViolence {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P4Beta1SafeSearchAnnotationViolence {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P4Beta1SafeSearchAnnotationViolence {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P4Beta1SafeSearchAnnotationViolence::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P4Beta1SafeSearchAnnotationViolence::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P4Beta1SafeSearchAnnotationViolence::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P4Beta1SafeSearchAnnotationViolence::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P4Beta1SafeSearchAnnotationViolence::VeryLikely,
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P4Beta1SafeSearchAnnotationViolence::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1SafeSearchAnnotationViolence {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1Symbol {
        #[doc = "The bounding box for the symbol.\nThe vertices are in the order of top-left, top-right, bottom-right,\nbottom-left. When a rotation of the bounding box is detected the rotation\nis represented as around the top-left corner as defined when the text is\nread in the 'natural' orientation.\nFor example:\n\n* when the text is horizontal it might look like:\n  0----1\n  |    |\n  3----2\n* when it's rotated 180 degrees around the top-left corner it becomes:\n  2----3\n  |    |\n  1----0\n  and the vertex order will still be (0, 1, 2, 3)."]
        #[serde(rename = "boundingBox", default)]
        pub bounding_box:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P4Beta1BoundingPoly>,
        #[doc = "Confidence of the OCR results for the symbol. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Additional information detected for the symbol."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P4Beta1TextAnnotationTextProperty,
        >,
        #[doc = "The actual UTF-8 representation of the symbol."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1Symbol {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1TextAnnotation {
        #[doc = "List of pages detected by OCR."]
        #[serde(rename = "pages", default)]
        pub pages: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P4Beta1Page>>,
        #[doc = "UTF-8 text detected on the pages."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1TextAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1TextAnnotationDetectedBreak {
        #[doc = "True if break prepends the element."]
        #[serde(rename = "isPrefix", default)]
        pub is_prefix: ::std::option::Option<bool>,
        #[doc = "Detected break type."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P4Beta1TextAnnotationDetectedBreakType,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1TextAnnotationDetectedBreak {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P4Beta1TextAnnotationDetectedBreakType {
        #[doc = "Line-wrapping break."]
        EolSureSpace,
        #[doc = "End-line hyphen that is not present in text; does not co-occur with\n`SPACE`, `LEADER_SPACE`, or `LINE_BREAK`."]
        Hyphen,
        #[doc = "Line break that ends a paragraph."]
        LineBreak,
        #[doc = "Regular space."]
        Space,
        #[doc = "Sure space (very wide)."]
        SureSpace,
        #[doc = "Unknown break label type."]
        Unknown,
    }
    impl GoogleCloudVisionV1P4Beta1TextAnnotationDetectedBreakType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P4Beta1TextAnnotationDetectedBreakType::EolSureSpace => {
                    "EOL_SURE_SPACE"
                }
                GoogleCloudVisionV1P4Beta1TextAnnotationDetectedBreakType::Hyphen => "HYPHEN",
                GoogleCloudVisionV1P4Beta1TextAnnotationDetectedBreakType::LineBreak => {
                    "LINE_BREAK"
                }
                GoogleCloudVisionV1P4Beta1TextAnnotationDetectedBreakType::Space => "SPACE",
                GoogleCloudVisionV1P4Beta1TextAnnotationDetectedBreakType::SureSpace => {
                    "SURE_SPACE"
                }
                GoogleCloudVisionV1P4Beta1TextAnnotationDetectedBreakType::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P4Beta1TextAnnotationDetectedBreakType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P4Beta1TextAnnotationDetectedBreakType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P4Beta1TextAnnotationDetectedBreakType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EOL_SURE_SPACE" => {
                    GoogleCloudVisionV1P4Beta1TextAnnotationDetectedBreakType::EolSureSpace
                }
                "HYPHEN" => GoogleCloudVisionV1P4Beta1TextAnnotationDetectedBreakType::Hyphen,
                "LINE_BREAK" => {
                    GoogleCloudVisionV1P4Beta1TextAnnotationDetectedBreakType::LineBreak
                }
                "SPACE" => GoogleCloudVisionV1P4Beta1TextAnnotationDetectedBreakType::Space,
                "SURE_SPACE" => {
                    GoogleCloudVisionV1P4Beta1TextAnnotationDetectedBreakType::SureSpace
                }
                "UNKNOWN" => GoogleCloudVisionV1P4Beta1TextAnnotationDetectedBreakType::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1TextAnnotationDetectedBreakType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1TextAnnotationDetectedLanguage {
        #[doc = "Confidence of detected language. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more\ninformation, see\nhttp://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1TextAnnotationDetectedLanguage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1TextAnnotationTextProperty {
        #[doc = "Detected start or end of a text segment."]
        #[serde(rename = "detectedBreak", default)]
        pub detected_break: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P4Beta1TextAnnotationDetectedBreak,
        >,
        #[doc = "A list of detected languages together with confidence."]
        #[serde(rename = "detectedLanguages", default)]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P4Beta1TextAnnotationDetectedLanguage>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1TextAnnotationTextProperty {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1Vertex {
        #[doc = "X coordinate."]
        #[serde(rename = "x", default)]
        pub x: ::std::option::Option<i32>,
        #[doc = "Y coordinate."]
        #[serde(rename = "y", default)]
        pub y: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1Vertex {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1WebDetection {
        #[doc = "The service's best guess as to the topic of the request image.\nInferred from similar images on the open web."]
        #[serde(rename = "bestGuessLabels", default)]
        pub best_guess_labels: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P4Beta1WebDetectionWebLabel>,
        >,
        #[doc = "Fully matching images from the Internet.\nCan include resized copies of the query image."]
        #[serde(rename = "fullMatchingImages", default)]
        pub full_matching_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P4Beta1WebDetectionWebImage>,
        >,
        #[doc = "Web pages containing the matching images from the Internet."]
        #[serde(rename = "pagesWithMatchingImages", default)]
        pub pages_with_matching_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P4Beta1WebDetectionWebPage>,
        >,
        #[doc = "Partial matching images from the Internet.\nThose images are similar enough to share some key-point features. For\nexample an original image will likely have partial matching for its crops."]
        #[serde(rename = "partialMatchingImages", default)]
        pub partial_matching_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P4Beta1WebDetectionWebImage>,
        >,
        #[doc = "The visually similar image results."]
        #[serde(rename = "visuallySimilarImages", default)]
        pub visually_similar_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P4Beta1WebDetectionWebImage>,
        >,
        #[doc = "Deduced entities from similar images on the Internet."]
        #[serde(rename = "webEntities", default)]
        pub web_entities: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P4Beta1WebDetectionWebEntity>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1WebDetection {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1WebDetectionWebEntity {
        #[doc = "Canonical description of the entity, in English."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Opaque entity ID."]
        #[serde(rename = "entityId", default)]
        pub entity_id: ::std::option::Option<String>,
        #[doc = "Overall relevancy score for the entity.\nNot normalized and not comparable across different image queries."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1WebDetectionWebEntity {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1WebDetectionWebImage {
        #[doc = "(Deprecated) Overall relevancy score for the image."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
        #[doc = "The result image URL."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1WebDetectionWebImage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1WebDetectionWebLabel {
        #[doc = "Label for extra metadata."]
        #[serde(rename = "label", default)]
        pub label: ::std::option::Option<String>,
        #[doc = "The BCP-47 language code for `label`, such as \"en-US\" or \"sr-Latn\".\nFor more information, see\nhttp://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1WebDetectionWebLabel {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1WebDetectionWebPage {
        #[doc = "Fully matching images on the page.\nCan include resized copies of the query image."]
        #[serde(rename = "fullMatchingImages", default)]
        pub full_matching_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P4Beta1WebDetectionWebImage>,
        >,
        #[doc = "Title for the web page, may contain HTML markups."]
        #[serde(rename = "pageTitle", default)]
        pub page_title: ::std::option::Option<String>,
        #[doc = "Partial matching images on the page.\nThose images are similar enough to share some key-point features. For\nexample an original image will likely have partial matching for its\ncrops."]
        #[serde(rename = "partialMatchingImages", default)]
        pub partial_matching_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P4Beta1WebDetectionWebImage>,
        >,
        #[doc = "(Deprecated) Overall relevancy score for the web page."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
        #[doc = "The result web page URL."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1WebDetectionWebPage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P4Beta1Word {
        #[doc = "The bounding box for the word.\nThe vertices are in the order of top-left, top-right, bottom-right,\nbottom-left. When a rotation of the bounding box is detected the rotation\nis represented as around the top-left corner as defined when the text is\nread in the 'natural' orientation.\nFor example:\n\n* when the text is horizontal it might look like:\n  0----1\n  |    |\n  3----2\n* when it's rotated 180 degrees around the top-left corner it becomes:\n  2----3\n  |    |\n  1----0\n  and the vertex order will still be (0, 1, 2, 3)."]
        #[serde(rename = "boundingBox", default)]
        pub bounding_box:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P4Beta1BoundingPoly>,
        #[doc = "Confidence of the OCR results for the word. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Additional information detected for the word."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P4Beta1TextAnnotationTextProperty,
        >,
        #[doc = "List of symbols in the word.\nThe order of the symbols follows the natural reading order."]
        #[serde(rename = "symbols", default)]
        pub symbols: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P4Beta1Symbol>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P4Beta1Word {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVisionV1P5Beta1AnnotateFileResponse {
        #[doc = "Information about the file for which this response is generated."]
        #[serde(rename = "inputConfig", default)]
        pub input_config:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1InputConfig>,
        #[doc = "Individual responses to images found within the file. This field will be\nempty if the `error` field is set."]
        #[serde(rename = "responses", default)]
        pub responses: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P5Beta1AnnotateImageResponse>,
        >,
        #[doc = "This field gives the total number of pages in the file."]
        #[serde(rename = "totalPages", default)]
        pub total_pages: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1AnnotateFileResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVisionV1P5Beta1AnnotateImageResponse {
        #[doc = "If present, contextual information is needed to understand where this image\ncomes from."]
        #[serde(rename = "context", default)]
        pub context:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1ImageAnnotationContext>,
        #[doc = "If present, crop hints have completed successfully."]
        #[serde(rename = "cropHintsAnnotation", default)]
        pub crop_hints_annotation:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1CropHintsAnnotation>,
        #[doc = "If set, represents the error message for the operation.\nNote that filled-in image annotations are guaranteed to be\ncorrect, even when `error` is set."]
        #[serde(rename = "error", default)]
        pub error: ::std::option::Option<crate::schemas::Status>,
        #[doc = "If present, face detection has completed successfully."]
        #[serde(rename = "faceAnnotations", default)]
        pub face_annotations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P5Beta1FaceAnnotation>>,
        #[doc = "If present, text (OCR) detection or document (OCR) text detection has\ncompleted successfully.\nThis annotation provides the structural hierarchy for the OCR detected\ntext."]
        #[serde(rename = "fullTextAnnotation", default)]
        pub full_text_annotation:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1TextAnnotation>,
        #[doc = "If present, image properties were extracted successfully."]
        #[serde(rename = "imagePropertiesAnnotation", default)]
        pub image_properties_annotation:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1ImageProperties>,
        #[doc = "If present, label detection has completed successfully."]
        #[serde(rename = "labelAnnotations", default)]
        pub label_annotations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P5Beta1EntityAnnotation>>,
        #[doc = "If present, landmark detection has completed successfully."]
        #[serde(rename = "landmarkAnnotations", default)]
        pub landmark_annotations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P5Beta1EntityAnnotation>>,
        #[doc = "If present, localized object detection has completed successfully.\nThis will be sorted descending by confidence score."]
        #[serde(rename = "localizedObjectAnnotations", default)]
        pub localized_object_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P5Beta1LocalizedObjectAnnotation>,
        >,
        #[doc = "If present, logo detection has completed successfully."]
        #[serde(rename = "logoAnnotations", default)]
        pub logo_annotations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P5Beta1EntityAnnotation>>,
        #[doc = "If present, product search has completed successfully."]
        #[serde(rename = "productSearchResults", default)]
        pub product_search_results:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1ProductSearchResults>,
        #[doc = "If present, safe-search annotation has completed successfully."]
        #[serde(rename = "safeSearchAnnotation", default)]
        pub safe_search_annotation:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1SafeSearchAnnotation>,
        #[doc = "If present, text (OCR) detection has completed successfully."]
        #[serde(rename = "textAnnotations", default)]
        pub text_annotations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P5Beta1EntityAnnotation>>,
        #[doc = "If present, web detection has completed successfully."]
        #[serde(rename = "webDetection", default)]
        pub web_detection:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1WebDetection>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1AnnotateImageResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1AsyncAnnotateFileResponse {
        #[doc = "The output location and metadata from AsyncAnnotateFileRequest."]
        #[serde(rename = "outputConfig", default)]
        pub output_config:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1OutputConfig>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1AsyncAnnotateFileResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1AsyncBatchAnnotateFilesResponse {
        #[doc = "The list of file annotation responses, one for each request in\nAsyncBatchAnnotateFilesRequest."]
        #[serde(rename = "responses", default)]
        pub responses: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P5Beta1AsyncAnnotateFileResponse>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1AsyncBatchAnnotateFilesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1AsyncBatchAnnotateImagesResponse {
        #[doc = "The output location and metadata from AsyncBatchAnnotateImagesRequest."]
        #[serde(rename = "outputConfig", default)]
        pub output_config:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1OutputConfig>,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVisionV1P5Beta1AsyncBatchAnnotateImagesResponse
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVisionV1P5Beta1BatchAnnotateFilesResponse {
        #[doc = "The list of file annotation responses, each response corresponding to each\nAnnotateFileRequest in BatchAnnotateFilesRequest."]
        #[serde(rename = "responses", default)]
        pub responses: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P5Beta1AnnotateFileResponse>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1BatchAnnotateFilesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1BatchOperationMetadata {
        #[doc = "The time when the batch request is finished and\ngoogle.longrunning.Operation.done is set to true."]
        #[serde(rename = "endTime", default)]
        pub end_time: ::std::option::Option<String>,
        #[doc = "The current state of the batch operation."]
        #[serde(rename = "state", default)]
        pub state: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P5Beta1BatchOperationMetadataState,
        >,
        #[doc = "The time when the batch request was submitted to the server."]
        #[serde(rename = "submitTime", default)]
        pub submit_time: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1BatchOperationMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P5Beta1BatchOperationMetadataState {
        #[doc = "The request is done after the longrunning.Operations.CancelOperation has\nbeen called by the user.  Any records that were processed before the\ncancel command are output as specified in the request."]
        Cancelled,
        #[doc = "The request is done and no item has been successfully processed."]
        Failed,
        #[doc = "Request is actively being processed."]
        Processing,
        #[doc = "Invalid."]
        StateUnspecified,
        #[doc = "The request is done and at least one item has been successfully\nprocessed."]
        Successful,
    }
    impl GoogleCloudVisionV1P5Beta1BatchOperationMetadataState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P5Beta1BatchOperationMetadataState::Cancelled => "CANCELLED",
                GoogleCloudVisionV1P5Beta1BatchOperationMetadataState::Failed => "FAILED",
                GoogleCloudVisionV1P5Beta1BatchOperationMetadataState::Processing => "PROCESSING",
                GoogleCloudVisionV1P5Beta1BatchOperationMetadataState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
                GoogleCloudVisionV1P5Beta1BatchOperationMetadataState::Successful => "SUCCESSFUL",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P5Beta1BatchOperationMetadataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P5Beta1BatchOperationMetadataState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P5Beta1BatchOperationMetadataState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => GoogleCloudVisionV1P5Beta1BatchOperationMetadataState::Cancelled,
                "FAILED" => GoogleCloudVisionV1P5Beta1BatchOperationMetadataState::Failed,
                "PROCESSING" => GoogleCloudVisionV1P5Beta1BatchOperationMetadataState::Processing,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudVisionV1P5Beta1BatchOperationMetadataState::StateUnspecified
                }
                "SUCCESSFUL" => GoogleCloudVisionV1P5Beta1BatchOperationMetadataState::Successful,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1BatchOperationMetadataState {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1Block {
        #[doc = "Detected block type (text, image etc) for this block."]
        #[serde(rename = "blockType", default)]
        pub block_type:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1BlockBlockType>,
        #[doc = "The bounding box for the block.\nThe vertices are in the order of top-left, top-right, bottom-right,\nbottom-left. When a rotation of the bounding box is detected the rotation\nis represented as around the top-left corner as defined when the text is\nread in the 'natural' orientation.\nFor example:\n\n* when the text is horizontal it might look like:\n  \n  ````text\n    0----1\n    |    |\n    3----2\n  ````\n\n* when it's rotated 180 degrees around the top-left corner it becomes:\n  \n  ````text\n    2----3\n    |    |\n    1----0\n  ````\n  \n  and the vertex order will still be (0, 1, 2, 3)."]
        #[serde(rename = "boundingBox", default)]
        pub bounding_box:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1BoundingPoly>,
        #[doc = "Confidence of the OCR results on the block. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Detected pair for KEY_VALUE_PAIR block_type. This detection can be turned\noff by explicitly setting desired fields in\nDocumentParsingParams.block_filter."]
        #[serde(rename = "keyValuePair", default)]
        pub key_value_pair:
            ::std::option::Option<Box<crate::schemas::GoogleCloudVisionV1P5Beta1KeyValuePair>>,
        #[doc = "All UTF-8 text detected in this block. This field is by default not\nreturned unless specified in TextDetectionParams.block_filter or\nDocumentParsingParams.block_filter."]
        #[serde(rename = "mergedText", default)]
        pub merged_text: ::std::option::Option<String>,
        #[doc = "List of paragraphs in this block (if this blocks is of type text)."]
        #[serde(rename = "paragraphs", default)]
        pub paragraphs:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P5Beta1Paragraph>>,
        #[doc = "Additional information detected for the block."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P5Beta1TextAnnotationTextProperty,
        >,
        #[doc = "Detected table for TABLE block_type. This detection can be turned off by\nexplicitly setting desired fields in DocumentParsingParams.block_filter."]
        #[serde(rename = "table", default)]
        pub table: ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1Table>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1Block {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P5Beta1BlockBlockType {
        #[doc = "Barcode block."]
        Barcode,
        #[doc = "A key-value pair block."]
        KeyValuePair,
        #[doc = "Image block."]
        Picture,
        #[doc = "Horizontal/vertical line box."]
        Ruler,
        #[doc = "Table block."]
        Table,
        #[doc = "Regular text block."]
        Text,
        #[doc = "Unknown block type."]
        Unknown,
    }
    impl GoogleCloudVisionV1P5Beta1BlockBlockType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P5Beta1BlockBlockType::Barcode => "BARCODE",
                GoogleCloudVisionV1P5Beta1BlockBlockType::KeyValuePair => "KEY_VALUE_PAIR",
                GoogleCloudVisionV1P5Beta1BlockBlockType::Picture => "PICTURE",
                GoogleCloudVisionV1P5Beta1BlockBlockType::Ruler => "RULER",
                GoogleCloudVisionV1P5Beta1BlockBlockType::Table => "TABLE",
                GoogleCloudVisionV1P5Beta1BlockBlockType::Text => "TEXT",
                GoogleCloudVisionV1P5Beta1BlockBlockType::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P5Beta1BlockBlockType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P5Beta1BlockBlockType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P5Beta1BlockBlockType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BARCODE" => GoogleCloudVisionV1P5Beta1BlockBlockType::Barcode,
                "KEY_VALUE_PAIR" => GoogleCloudVisionV1P5Beta1BlockBlockType::KeyValuePair,
                "PICTURE" => GoogleCloudVisionV1P5Beta1BlockBlockType::Picture,
                "RULER" => GoogleCloudVisionV1P5Beta1BlockBlockType::Ruler,
                "TABLE" => GoogleCloudVisionV1P5Beta1BlockBlockType::Table,
                "TEXT" => GoogleCloudVisionV1P5Beta1BlockBlockType::Text,
                "UNKNOWN" => GoogleCloudVisionV1P5Beta1BlockBlockType::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1BlockBlockType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1BoundingPoly {
        #[doc = "The bounding polygon normalized vertices."]
        #[serde(rename = "normalizedVertices", default)]
        pub normalized_vertices:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P5Beta1NormalizedVertex>>,
        #[doc = "The bounding polygon vertices."]
        #[serde(rename = "vertices", default)]
        pub vertices: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P5Beta1Vertex>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1BoundingPoly {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1ColorInfo {
        #[doc = "RGB components of the color."]
        #[serde(rename = "color", default)]
        pub color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "The fraction of pixels the color occupies in the image.\nValue in range [0, 1]."]
        #[serde(rename = "pixelFraction", default)]
        pub pixel_fraction: ::std::option::Option<f32>,
        #[doc = "Image-specific score for this color. Value in range [0, 1]."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1ColorInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1CropHint {
        #[doc = "The bounding polygon for the crop region. The coordinates of the bounding\nbox are in the original image's scale."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1BoundingPoly>,
        #[doc = "Confidence of this being a salient region.  Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Fraction of importance of this salient region with respect to the original\nimage."]
        #[serde(rename = "importanceFraction", default)]
        pub importance_fraction: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1CropHint {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1CropHintsAnnotation {
        #[doc = "Crop hint results."]
        #[serde(rename = "cropHints", default)]
        pub crop_hints:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P5Beta1CropHint>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1CropHintsAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1DominantColorsAnnotation {
        #[doc = "RGB color values with their score and pixel fraction."]
        #[serde(rename = "colors", default)]
        pub colors: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P5Beta1ColorInfo>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1DominantColorsAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1EntityAnnotation {
        #[doc = "Image region to which this entity belongs. Not produced\nfor `LABEL_DETECTION` features."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1BoundingPoly>,
        #[doc = "**Deprecated. Use `score` instead.**\nThe accuracy of the entity detection in an image.\nFor example, for an image in which the \"Eiffel Tower\" entity is detected,\nthis field represents the confidence that there is a tower in the query\nimage. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Entity textual description, expressed in its `locale` language."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "The language code for the locale in which the entity textual\n`description` is expressed."]
        #[serde(rename = "locale", default)]
        pub locale: ::std::option::Option<String>,
        #[doc = "The location information for the detected entity. Multiple\n`LocationInfo` elements can be present because one location may\nindicate the location of the scene in the image, and another location\nmay indicate the location of the place where the image was taken.\nLocation information is usually present for landmarks."]
        #[serde(rename = "locations", default)]
        pub locations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P5Beta1LocationInfo>>,
        #[doc = "Opaque entity ID. Some IDs may be available in\n[Google Knowledge Graph Search\nAPI](https://developers.google.com/knowledge-graph/)."]
        #[serde(rename = "mid", default)]
        pub mid: ::std::option::Option<String>,
        #[doc = "Some entities may have optional user-supplied `Property` (name/value)\nfields, such a score or string that qualifies the entity."]
        #[serde(rename = "properties", default)]
        pub properties:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P5Beta1Property>>,
        #[doc = "Overall score of the result. Range [0, 1]."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
        #[doc = "The relevancy of the ICA (Image Content Annotation) label to the\nimage. For example, the relevancy of \"tower\" is likely higher to an image\ncontaining the detected \"Eiffel Tower\" than to an image containing a\ndetected distant towering building, even though the confidence that\nthere is a tower in each image may be the same. Range [0, 1]."]
        #[serde(rename = "topicality", default)]
        pub topicality: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1EntityAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1FaceAnnotation {
        #[doc = "Anger likelihood."]
        #[serde(rename = "angerLikelihood", default)]
        pub anger_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P5Beta1FaceAnnotationAngerLikelihood,
        >,
        #[doc = "Blurred likelihood."]
        #[serde(rename = "blurredLikelihood", default)]
        pub blurred_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P5Beta1FaceAnnotationBlurredLikelihood,
        >,
        #[doc = "The bounding polygon around the face. The coordinates of the bounding box\nare in the original image's scale.\nThe bounding box is computed to \"frame\" the face in accordance with human\nexpectations. It is based on the landmarker results.\nNote that one or more x and/or y coordinates may not be generated in the\n`BoundingPoly` (the polygon will be unbounded) if only a partial face\nappears in the image to be annotated."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1BoundingPoly>,
        #[doc = "Detection confidence. Range [0, 1]."]
        #[serde(rename = "detectionConfidence", default)]
        pub detection_confidence: ::std::option::Option<f32>,
        #[doc = "The `fd_bounding_poly` bounding polygon is tighter than the\n`boundingPoly`, and encloses only the skin part of the face. Typically, it\nis used to eliminate the face from any image analysis that detects the\n\"amount of skin\" visible in an image. It is not based on the\nlandmarker results, only on the initial face detection, hence\nthe <code>fd</code> (face detection) prefix."]
        #[serde(rename = "fdBoundingPoly", default)]
        pub fd_bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1BoundingPoly>,
        #[doc = "Headwear likelihood."]
        #[serde(rename = "headwearLikelihood", default)]
        pub headwear_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P5Beta1FaceAnnotationHeadwearLikelihood,
        >,
        #[doc = "Joy likelihood."]
        #[serde(rename = "joyLikelihood", default)]
        pub joy_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P5Beta1FaceAnnotationJoyLikelihood,
        >,
        #[doc = "Face landmarking confidence. Range [0, 1]."]
        #[serde(rename = "landmarkingConfidence", default)]
        pub landmarking_confidence: ::std::option::Option<f32>,
        #[doc = "Detected face landmarks."]
        #[serde(rename = "landmarks", default)]
        pub landmarks: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P5Beta1FaceAnnotationLandmark>,
        >,
        #[doc = "Yaw angle, which indicates the leftward/rightward angle that the face is\npointing relative to the vertical plane perpendicular to the image. Range\n[-180,180]."]
        #[serde(rename = "panAngle", default)]
        pub pan_angle: ::std::option::Option<f32>,
        #[doc = "Roll angle, which indicates the amount of clockwise/anti-clockwise rotation\nof the face relative to the image vertical about the axis perpendicular to\nthe face. Range [-180,180]."]
        #[serde(rename = "rollAngle", default)]
        pub roll_angle: ::std::option::Option<f32>,
        #[doc = "Sorrow likelihood."]
        #[serde(rename = "sorrowLikelihood", default)]
        pub sorrow_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P5Beta1FaceAnnotationSorrowLikelihood,
        >,
        #[doc = "Surprise likelihood."]
        #[serde(rename = "surpriseLikelihood", default)]
        pub surprise_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P5Beta1FaceAnnotationSurpriseLikelihood,
        >,
        #[doc = "Pitch angle, which indicates the upwards/downwards angle that the face is\npointing relative to the image's horizontal plane. Range [-180,180]."]
        #[serde(rename = "tiltAngle", default)]
        pub tilt_angle: ::std::option::Option<f32>,
        #[doc = "Under-exposed likelihood."]
        #[serde(rename = "underExposedLikelihood", default)]
        pub under_exposed_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P5Beta1FaceAnnotationUnderExposedLikelihood,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1FaceAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P5Beta1FaceAnnotationAngerLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P5Beta1FaceAnnotationAngerLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P5Beta1FaceAnnotationAngerLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P5Beta1FaceAnnotationAngerLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P5Beta1FaceAnnotationAngerLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P5Beta1FaceAnnotationAngerLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P5Beta1FaceAnnotationAngerLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationAngerLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P5Beta1FaceAnnotationAngerLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P5Beta1FaceAnnotationAngerLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P5Beta1FaceAnnotationAngerLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P5Beta1FaceAnnotationAngerLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P5Beta1FaceAnnotationAngerLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P5Beta1FaceAnnotationAngerLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P5Beta1FaceAnnotationAngerLikelihood::Unlikely,
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationAngerLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationAngerLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1FaceAnnotationAngerLikelihood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P5Beta1FaceAnnotationBlurredLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P5Beta1FaceAnnotationBlurredLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P5Beta1FaceAnnotationBlurredLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P5Beta1FaceAnnotationBlurredLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P5Beta1FaceAnnotationBlurredLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P5Beta1FaceAnnotationBlurredLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P5Beta1FaceAnnotationBlurredLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationBlurredLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P5Beta1FaceAnnotationBlurredLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P5Beta1FaceAnnotationBlurredLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P5Beta1FaceAnnotationBlurredLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P5Beta1FaceAnnotationBlurredLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P5Beta1FaceAnnotationBlurredLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P5Beta1FaceAnnotationBlurredLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P5Beta1FaceAnnotationBlurredLikelihood::Unlikely,
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationBlurredLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationBlurredLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1FaceAnnotationBlurredLikelihood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P5Beta1FaceAnnotationHeadwearLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P5Beta1FaceAnnotationHeadwearLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P5Beta1FaceAnnotationHeadwearLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P5Beta1FaceAnnotationHeadwearLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P5Beta1FaceAnnotationHeadwearLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P5Beta1FaceAnnotationHeadwearLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P5Beta1FaceAnnotationHeadwearLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationHeadwearLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P5Beta1FaceAnnotationHeadwearLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P5Beta1FaceAnnotationHeadwearLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P5Beta1FaceAnnotationHeadwearLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P5Beta1FaceAnnotationHeadwearLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P5Beta1FaceAnnotationHeadwearLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P5Beta1FaceAnnotationHeadwearLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P5Beta1FaceAnnotationHeadwearLikelihood::Unlikely,
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationHeadwearLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationHeadwearLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVisionV1P5Beta1FaceAnnotationHeadwearLikelihood
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P5Beta1FaceAnnotationJoyLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P5Beta1FaceAnnotationJoyLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P5Beta1FaceAnnotationJoyLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P5Beta1FaceAnnotationJoyLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P5Beta1FaceAnnotationJoyLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P5Beta1FaceAnnotationJoyLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P5Beta1FaceAnnotationJoyLikelihood::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P5Beta1FaceAnnotationJoyLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P5Beta1FaceAnnotationJoyLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P5Beta1FaceAnnotationJoyLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P5Beta1FaceAnnotationJoyLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P5Beta1FaceAnnotationJoyLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P5Beta1FaceAnnotationJoyLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P5Beta1FaceAnnotationJoyLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P5Beta1FaceAnnotationJoyLikelihood::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P5Beta1FaceAnnotationJoyLikelihood::VeryLikely,
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationJoyLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1FaceAnnotationJoyLikelihood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P5Beta1FaceAnnotationSorrowLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P5Beta1FaceAnnotationSorrowLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P5Beta1FaceAnnotationSorrowLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P5Beta1FaceAnnotationSorrowLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P5Beta1FaceAnnotationSorrowLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P5Beta1FaceAnnotationSorrowLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P5Beta1FaceAnnotationSorrowLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationSorrowLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P5Beta1FaceAnnotationSorrowLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P5Beta1FaceAnnotationSorrowLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P5Beta1FaceAnnotationSorrowLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P5Beta1FaceAnnotationSorrowLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P5Beta1FaceAnnotationSorrowLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P5Beta1FaceAnnotationSorrowLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P5Beta1FaceAnnotationSorrowLikelihood::Unlikely,
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationSorrowLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationSorrowLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1FaceAnnotationSorrowLikelihood {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P5Beta1FaceAnnotationSurpriseLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P5Beta1FaceAnnotationSurpriseLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P5Beta1FaceAnnotationSurpriseLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P5Beta1FaceAnnotationSurpriseLikelihood::Possible => "POSSIBLE",
                GoogleCloudVisionV1P5Beta1FaceAnnotationSurpriseLikelihood::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P5Beta1FaceAnnotationSurpriseLikelihood::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P5Beta1FaceAnnotationSurpriseLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationSurpriseLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P5Beta1FaceAnnotationSurpriseLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P5Beta1FaceAnnotationSurpriseLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P5Beta1FaceAnnotationSurpriseLikelihood {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P5Beta1FaceAnnotationSurpriseLikelihood::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P5Beta1FaceAnnotationSurpriseLikelihood::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P5Beta1FaceAnnotationSurpriseLikelihood::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P5Beta1FaceAnnotationSurpriseLikelihood::Unlikely,
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationSurpriseLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationSurpriseLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVisionV1P5Beta1FaceAnnotationSurpriseLikelihood
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P5Beta1FaceAnnotationUnderExposedLikelihood {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P5Beta1FaceAnnotationUnderExposedLikelihood {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P5Beta1FaceAnnotationUnderExposedLikelihood::Likely => "LIKELY",
                GoogleCloudVisionV1P5Beta1FaceAnnotationUnderExposedLikelihood::Possible => {
                    "POSSIBLE"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationUnderExposedLikelihood::Unknown => {
                    "UNKNOWN"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationUnderExposedLikelihood::Unlikely => {
                    "UNLIKELY"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationUnderExposedLikelihood::VeryLikely => {
                    "VERY_LIKELY"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationUnderExposedLikelihood::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P5Beta1FaceAnnotationUnderExposedLikelihood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P5Beta1FaceAnnotationUnderExposedLikelihood {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVisionV1P5Beta1FaceAnnotationUnderExposedLikelihood
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P5Beta1FaceAnnotationUnderExposedLikelihood::Likely,
                "POSSIBLE" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationUnderExposedLikelihood::Possible
                }
                "UNKNOWN" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationUnderExposedLikelihood::Unknown
                }
                "UNLIKELY" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationUnderExposedLikelihood::Unlikely
                }
                "VERY_LIKELY" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationUnderExposedLikelihood::VeryLikely
                }
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationUnderExposedLikelihood::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVisionV1P5Beta1FaceAnnotationUnderExposedLikelihood
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1FaceAnnotationLandmark {
        #[doc = "Face landmark position."]
        #[serde(rename = "position", default)]
        pub position: ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1Position>,
        #[doc = "Face landmark type."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1FaceAnnotationLandmark {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType {
        #[doc = "Chin gnathion."]
        ChinGnathion,
        #[doc = "Chin left gonion."]
        ChinLeftGonion,
        #[doc = "Chin right gonion."]
        ChinRightGonion,
        #[doc = "Forehead glabella."]
        ForeheadGlabella,
        #[doc = "Left ear tragion."]
        LeftEarTragion,
        #[doc = "Left eye."]
        LeftEye,
        #[doc = "Left eye, bottom boundary."]
        LeftEyeBottomBoundary,
        #[doc = "Left eye, left corner."]
        LeftEyeLeftCorner,
        #[doc = "Left eye pupil."]
        LeftEyePupil,
        #[doc = "Left eye, right corner."]
        LeftEyeRightCorner,
        #[doc = "Left eye, top boundary."]
        LeftEyeTopBoundary,
        #[doc = "Left eyebrow, upper midpoint."]
        LeftEyebrowUpperMidpoint,
        #[doc = "Left of left eyebrow."]
        LeftOfLeftEyebrow,
        #[doc = "Left of right eyebrow."]
        LeftOfRightEyebrow,
        #[doc = "Lower lip."]
        LowerLip,
        #[doc = "Midpoint between eyes."]
        MidpointBetweenEyes,
        #[doc = "Mouth center."]
        MouthCenter,
        #[doc = "Mouth left."]
        MouthLeft,
        #[doc = "Mouth right."]
        MouthRight,
        #[doc = "Nose, bottom center."]
        NoseBottomCenter,
        #[doc = "Nose, bottom left."]
        NoseBottomLeft,
        #[doc = "Nose, bottom right."]
        NoseBottomRight,
        #[doc = "Nose tip."]
        NoseTip,
        #[doc = "Right ear tragion."]
        RightEarTragion,
        #[doc = "Right eye."]
        RightEye,
        #[doc = "Right eye, bottom boundary."]
        RightEyeBottomBoundary,
        #[doc = "Right eye, left corner."]
        RightEyeLeftCorner,
        #[doc = "Right eye pupil."]
        RightEyePupil,
        #[doc = "Right eye, right corner."]
        RightEyeRightCorner,
        #[doc = "Right eye, top boundary."]
        RightEyeTopBoundary,
        #[doc = "Right eyebrow, upper midpoint."]
        RightEyebrowUpperMidpoint,
        #[doc = "Right of left eyebrow."]
        RightOfLeftEyebrow,
        #[doc = "Right of right eyebrow."]
        RightOfRightEyebrow,
        #[doc = "Unknown face landmark detected. Should not be filled."]
        UnknownLandmark,
        #[doc = "Upper lip."]
        UpperLip,
    }
    impl GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::ChinGnathion => {
                    "CHIN_GNATHION"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::ChinLeftGonion => {
                    "CHIN_LEFT_GONION"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::ChinRightGonion => {
                    "CHIN_RIGHT_GONION"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::ForeheadGlabella => {
                    "FOREHEAD_GLABELLA"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::LeftEarTragion => {
                    "LEFT_EAR_TRAGION"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::LeftEye => "LEFT_EYE",
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::LeftEyeBottomBoundary => {
                    "LEFT_EYE_BOTTOM_BOUNDARY"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::LeftEyeLeftCorner => {
                    "LEFT_EYE_LEFT_CORNER"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::LeftEyePupil => {
                    "LEFT_EYE_PUPIL"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::LeftEyeRightCorner => {
                    "LEFT_EYE_RIGHT_CORNER"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::LeftEyeTopBoundary => {
                    "LEFT_EYE_TOP_BOUNDARY"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::LeftEyebrowUpperMidpoint => {
                    "LEFT_EYEBROW_UPPER_MIDPOINT"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::LeftOfLeftEyebrow => {
                    "LEFT_OF_LEFT_EYEBROW"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::LeftOfRightEyebrow => {
                    "LEFT_OF_RIGHT_EYEBROW"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::LowerLip => "LOWER_LIP",
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::MidpointBetweenEyes => {
                    "MIDPOINT_BETWEEN_EYES"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::MouthCenter => "MOUTH_CENTER",
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::MouthLeft => "MOUTH_LEFT",
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::MouthRight => "MOUTH_RIGHT",
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::NoseBottomCenter => {
                    "NOSE_BOTTOM_CENTER"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::NoseBottomLeft => {
                    "NOSE_BOTTOM_LEFT"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::NoseBottomRight => {
                    "NOSE_BOTTOM_RIGHT"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::NoseTip => "NOSE_TIP",
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::RightEarTragion => {
                    "RIGHT_EAR_TRAGION"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::RightEye => "RIGHT_EYE",
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::RightEyeBottomBoundary => {
                    "RIGHT_EYE_BOTTOM_BOUNDARY"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::RightEyeLeftCorner => {
                    "RIGHT_EYE_LEFT_CORNER"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::RightEyePupil => {
                    "RIGHT_EYE_PUPIL"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::RightEyeRightCorner => {
                    "RIGHT_EYE_RIGHT_CORNER"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::RightEyeTopBoundary => {
                    "RIGHT_EYE_TOP_BOUNDARY"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::RightEyebrowUpperMidpoint => {
                    "RIGHT_EYEBROW_UPPER_MIDPOINT"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::RightOfLeftEyebrow => {
                    "RIGHT_OF_LEFT_EYEBROW"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::RightOfRightEyebrow => {
                    "RIGHT_OF_RIGHT_EYEBROW"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::UnknownLandmark => {
                    "UNKNOWN_LANDMARK"
                }
                GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::UpperLip => "UPPER_LIP",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CHIN_GNATHION" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::ChinGnathion
                }
                "CHIN_LEFT_GONION" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::ChinLeftGonion
                }
                "CHIN_RIGHT_GONION" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::ChinRightGonion
                }
                "FOREHEAD_GLABELLA" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::ForeheadGlabella
                }
                "LEFT_EAR_TRAGION" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::LeftEarTragion
                }
                "LEFT_EYE" => GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::LeftEye,
                "LEFT_EYE_BOTTOM_BOUNDARY" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::LeftEyeBottomBoundary
                }
                "LEFT_EYE_LEFT_CORNER" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::LeftEyeLeftCorner
                }
                "LEFT_EYE_PUPIL" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::LeftEyePupil
                }
                "LEFT_EYE_RIGHT_CORNER" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::LeftEyeRightCorner
                }
                "LEFT_EYE_TOP_BOUNDARY" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::LeftEyeTopBoundary
                }
                "LEFT_EYEBROW_UPPER_MIDPOINT" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::LeftEyebrowUpperMidpoint
                }
                "LEFT_OF_LEFT_EYEBROW" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::LeftOfLeftEyebrow
                }
                "LEFT_OF_RIGHT_EYEBROW" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::LeftOfRightEyebrow
                }
                "LOWER_LIP" => GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::LowerLip,
                "MIDPOINT_BETWEEN_EYES" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::MidpointBetweenEyes
                }
                "MOUTH_CENTER" => GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::MouthCenter,
                "MOUTH_LEFT" => GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::MouthLeft,
                "MOUTH_RIGHT" => GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::MouthRight,
                "NOSE_BOTTOM_CENTER" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::NoseBottomCenter
                }
                "NOSE_BOTTOM_LEFT" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::NoseBottomLeft
                }
                "NOSE_BOTTOM_RIGHT" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::NoseBottomRight
                }
                "NOSE_TIP" => GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::NoseTip,
                "RIGHT_EAR_TRAGION" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::RightEarTragion
                }
                "RIGHT_EYE" => GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::RightEye,
                "RIGHT_EYE_BOTTOM_BOUNDARY" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::RightEyeBottomBoundary
                }
                "RIGHT_EYE_LEFT_CORNER" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::RightEyeLeftCorner
                }
                "RIGHT_EYE_PUPIL" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::RightEyePupil
                }
                "RIGHT_EYE_RIGHT_CORNER" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::RightEyeRightCorner
                }
                "RIGHT_EYE_TOP_BOUNDARY" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::RightEyeTopBoundary
                }
                "RIGHT_EYEBROW_UPPER_MIDPOINT" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::RightEyebrowUpperMidpoint
                }
                "RIGHT_OF_LEFT_EYEBROW" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::RightOfLeftEyebrow
                }
                "RIGHT_OF_RIGHT_EYEBROW" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::RightOfRightEyebrow
                }
                "UNKNOWN_LANDMARK" => {
                    GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::UnknownLandmark
                }
                "UPPER_LIP" => GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType::UpperLip,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1FaceAnnotationLandmarkType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1GcsDestination {
        #[doc = "Google Cloud Storage URI prefix where the results will be stored. Results\nwill be in JSON format and preceded by its corresponding input URI prefix.\nThis field can either represent a gcs file prefix or gcs directory. In\neither case, the uri should be unique because in order to get all of the\noutput files, you will need to do a wildcard gcs search on the uri prefix\nyou provide.\n\nExamples:\n\n* File Prefix: gs://bucket-name/here/filenameprefix   The output files\n  will be created in gs://bucket-name/here/ and the names of the\n  output files will begin with \"filenameprefix\".\n\n* Directory Prefix: gs://bucket-name/some/location/   The output files\n  will be created in gs://bucket-name/some/location/ and the names of the\n  output files could be anything because there was no filename prefix\n  specified.\n\nIf multiple outputs, each response is still AnnotateFileResponse, each of\nwhich contains some subset of the full list of AnnotateImageResponse.\nMultiple outputs can happen if, for example, the output JSON is too large\nand overflows into multiple sharded files."]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1GcsDestination {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1GcsSource {
        #[doc = "Google Cloud Storage URI for the input file. This must only be a\nGoogle Cloud Storage object. Wildcards are not currently supported."]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1GcsSource {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1ImageAnnotationContext {
        #[doc = "If the file was a PDF or TIFF, this field gives the page number within\nthe file used to produce the image."]
        #[serde(rename = "pageNumber", default)]
        pub page_number: ::std::option::Option<i32>,
        #[doc = "The URI of the file used to produce the image."]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1ImageAnnotationContext {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1ImageProperties {
        #[doc = "If present, dominant colors completed successfully."]
        #[serde(rename = "dominantColors", default)]
        pub dominant_colors: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P5Beta1DominantColorsAnnotation,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1ImageProperties {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVisionV1P5Beta1ImportProductSetsResponse {
        #[doc = "The list of reference_images that are imported successfully."]
        #[serde(rename = "referenceImages", default)]
        pub reference_images:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P5Beta1ReferenceImage>>,
        #[doc = "The rpc status for each ImportProductSet request, including both successes\nand errors.\n\nThe number of statuses here matches the number of lines in the csv file,\nand statuses[i] stores the success or failure status of processing the i-th\nline of the csv, starting from line 0."]
        #[serde(rename = "statuses", default)]
        pub statuses: ::std::option::Option<Vec<crate::schemas::Status>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1ImportProductSetsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1InputConfig {
        #[doc = "File content, represented as a stream of bytes.\nNote: As with all `bytes` fields, protobuffers use a pure binary\nrepresentation, whereas JSON representations use base64.\n\nCurrently, this field only works for BatchAnnotateFiles requests. It does\nnot work for AsyncBatchAnnotateFiles requests."]
        #[serde(rename = "content", default)]
        pub content: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "The Google Cloud Storage location to read the input from."]
        #[serde(rename = "gcsSource", default)]
        pub gcs_source: ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1GcsSource>,
        #[doc = "The type of the file. Currently only \"application/pdf\", \"image/tiff\" and\n\"image/gif\" are supported. Wildcards are not supported."]
        #[serde(rename = "mimeType", default)]
        pub mime_type: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1InputConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1KeyValuePair {
        #[doc = "The key string value."]
        #[serde(rename = "key", default)]
        pub key: ::std::option::Option<String>,
        #[doc = "Key block of the pair containing the normalized bounding box and key text."]
        #[serde(rename = "keyBlock", default)]
        pub key_block: ::std::option::Option<Box<crate::schemas::GoogleCloudVisionV1P5Beta1Block>>,
        #[doc = "Optional. The translation of key text if the text is not in English."]
        #[serde(rename = "normalizedKey", default)]
        pub normalized_key: ::std::option::Option<String>,
        #[doc = "Value block of the pair containing the normalized bounding box and value\ntext, including potentially deeper structures within the value text."]
        #[serde(rename = "valueBlock", default)]
        pub value_block:
            ::std::option::Option<Box<crate::schemas::GoogleCloudVisionV1P5Beta1Block>>,
        #[doc = "Type of the value. Valid strings are the following:\n\n\"generic\" - For generic text that is mapped to a value.\n\"number\" - for numeric types\n\"id\" - for generic identifiers.\n\"currency\" - for currency values.\n\"date\" - for dates.\n\"time\" - for time and duration values.\n\"date_range\" - for date ranges.\n\"address\" - for address values (can be long).\n\"person\" - for names of people or other personal identifiers.\n\"phone\" - for phone numbers."]
        #[serde(rename = "valueType", default)]
        pub value_type: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1KeyValuePair {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1LocalizedObjectAnnotation {
        #[doc = "Image region to which this object belongs. This must be populated."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1BoundingPoly>,
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more\ninformation, see\nhttp://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Object ID that should align with EntityAnnotation mid."]
        #[serde(rename = "mid", default)]
        pub mid: ::std::option::Option<String>,
        #[doc = "Object name, expressed in its `language_code` language."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Score of the result. Range [0, 1]."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1LocalizedObjectAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1LocationInfo {
        #[doc = "lat/long location coordinates."]
        #[serde(rename = "latLng", default)]
        pub lat_lng: ::std::option::Option<crate::schemas::LatLng>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1LocationInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1NormalizedVertex {
        #[doc = "X coordinate."]
        #[serde(rename = "x", default)]
        pub x: ::std::option::Option<f32>,
        #[doc = "Y coordinate."]
        #[serde(rename = "y", default)]
        pub y: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1NormalizedVertex {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1OperationMetadata {
        #[doc = "The time when the batch request was received."]
        #[serde(rename = "createTime", default)]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Current state of the batch operation."]
        #[serde(rename = "state", default)]
        pub state:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1OperationMetadataState>,
        #[doc = "The time when the operation result was last updated."]
        #[serde(rename = "updateTime", default)]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1OperationMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P5Beta1OperationMetadataState {
        #[doc = "The batch processing was cancelled."]
        Cancelled,
        #[doc = "Request is received."]
        Created,
        #[doc = "The batch processing is done."]
        Done,
        #[doc = "Request is actively being processed."]
        Running,
        #[doc = "Invalid."]
        StateUnspecified,
    }
    impl GoogleCloudVisionV1P5Beta1OperationMetadataState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P5Beta1OperationMetadataState::Cancelled => "CANCELLED",
                GoogleCloudVisionV1P5Beta1OperationMetadataState::Created => "CREATED",
                GoogleCloudVisionV1P5Beta1OperationMetadataState::Done => "DONE",
                GoogleCloudVisionV1P5Beta1OperationMetadataState::Running => "RUNNING",
                GoogleCloudVisionV1P5Beta1OperationMetadataState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P5Beta1OperationMetadataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P5Beta1OperationMetadataState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P5Beta1OperationMetadataState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => GoogleCloudVisionV1P5Beta1OperationMetadataState::Cancelled,
                "CREATED" => GoogleCloudVisionV1P5Beta1OperationMetadataState::Created,
                "DONE" => GoogleCloudVisionV1P5Beta1OperationMetadataState::Done,
                "RUNNING" => GoogleCloudVisionV1P5Beta1OperationMetadataState::Running,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudVisionV1P5Beta1OperationMetadataState::StateUnspecified
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1OperationMetadataState {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1OutputConfig {
        #[doc = "The max number of response protos to put into each output JSON file on\nGoogle Cloud Storage.\nThe valid range is [1, 100]. If not specified, the default value is 20.\n\nFor example, for one pdf file with 100 pages, 100 response protos will\nbe generated. If `batch_size` = 20, then 5 json files each\ncontaining 20 response protos will be written under the prefix\n`gcs_destination`.`uri`.\n\nCurrently, batch_size only applies to GcsDestination, with potential future\nsupport for other output configurations."]
        #[serde(rename = "batchSize", default)]
        pub batch_size: ::std::option::Option<i32>,
        #[doc = "The Google Cloud Storage location to write the output(s) to."]
        #[serde(rename = "gcsDestination", default)]
        pub gcs_destination:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1GcsDestination>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1OutputConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1Page {
        #[doc = "List of blocks of text, images etc on this page."]
        #[serde(rename = "blocks", default)]
        pub blocks: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P5Beta1Block>>,
        #[doc = "Confidence of the OCR results on the page. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Page height. For PDFs the unit is points. For images (including\nTIFFs) the unit is pixels."]
        #[serde(rename = "height", default)]
        pub height: ::std::option::Option<i32>,
        #[doc = "All UTF-8 text detected in this page. This field is by default not\nreturned unless specified in TextDetectionParams.page_filter."]
        #[serde(rename = "mergedText", default)]
        pub merged_text: ::std::option::Option<String>,
        #[doc = "Additional information detected on the page."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P5Beta1TextAnnotationTextProperty,
        >,
        #[doc = "Page width. For PDFs the unit is points. For images (including\nTIFFs) the unit is pixels."]
        #[serde(rename = "width", default)]
        pub width: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1Page {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1Paragraph {
        #[doc = "The bounding box for the paragraph.\nThe vertices are in the order of top-left, top-right, bottom-right,\nbottom-left. When a rotation of the bounding box is detected the rotation\nis represented as around the top-left corner as defined when the text is\nread in the 'natural' orientation.\nFor example:\n\n* when the text is horizontal it might look like:\n  0----1\n  |    |\n  3----2\n* when it's rotated 180 degrees around the top-left corner it becomes:\n  2----3\n  |    |\n  1----0\n  and the vertex order will still be (0, 1, 2, 3)."]
        #[serde(rename = "boundingBox", default)]
        pub bounding_box:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1BoundingPoly>,
        #[doc = "Confidence of the OCR results for the paragraph. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "All UTF-8 text detected in this paragraph. This field is by default not\nreturned unless specified in TextDetectionParams.paragraph_filter."]
        #[serde(rename = "mergedText", default)]
        pub merged_text: ::std::option::Option<String>,
        #[doc = "Additional information detected for the paragraph."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P5Beta1TextAnnotationTextProperty,
        >,
        #[doc = "List of all words in this paragraph."]
        #[serde(rename = "words", default)]
        pub words: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P5Beta1Word>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1Paragraph {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1Position {
        #[doc = "X coordinate."]
        #[serde(rename = "x", default)]
        pub x: ::std::option::Option<f32>,
        #[doc = "Y coordinate."]
        #[serde(rename = "y", default)]
        pub y: ::std::option::Option<f32>,
        #[doc = "Z coordinate (or depth)."]
        #[serde(rename = "z", default)]
        pub z: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1Position {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1Product {
        #[doc = "User-provided metadata to be stored with this product. Must be at most 4096\ncharacters long."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "The user-provided name for this Product. Must not be empty. Must be at most\n4096 characters long."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The resource name of the product.\n\nFormat is:\n`projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`.\n\nThis field is ignored when creating a product."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The category for the product identified by the reference image. This should\nbe either \"homegoods-v2\", \"apparel-v2\", or \"toys-v2\". The legacy categories\n\"homegoods\", \"apparel\", and \"toys\" are still supported, but these should\nnot be used for new products.\n\nThis field is immutable."]
        #[serde(rename = "productCategory", default)]
        pub product_category: ::std::option::Option<String>,
        #[doc = "Key-value pairs that can be attached to a product. At query time,\nconstraints can be specified based on the product_labels.\n\nNote that integer values can be provided as strings, e.g. \"1199\". Only\nstrings with integer values can match a range-based restriction which is\nto be supported soon.\n\nMultiple values can be assigned to the same key. One product may have up to\n500 product_labels.\n\nNotice that the total number of distinct product_labels over all products\nin one ProductSet cannot exceed 1M, otherwise the product search pipeline\nwill refuse to work for that ProductSet."]
        #[serde(rename = "productLabels", default)]
        pub product_labels:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P5Beta1ProductKeyValue>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1Product {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1ProductKeyValue {
        #[doc = "The key of the label attached to the product. Cannot be empty and cannot\nexceed 128 bytes."]
        #[serde(rename = "key", default)]
        pub key: ::std::option::Option<String>,
        #[doc = "The value of the label attached to the product. Cannot be empty and\ncannot exceed 128 bytes."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1ProductKeyValue {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1ProductSearchResults {
        #[doc = "Timestamp of the index which provided these results. Products added to the\nproduct set and products removed from the product set after this time are\nnot reflected in the current results."]
        #[serde(rename = "indexTime", default)]
        pub index_time: ::std::option::Option<String>,
        #[doc = "List of results grouped by products detected in the query image. Each entry\ncorresponds to one bounding polygon in the query image, and contains the\nmatching products specific to that region. There may be duplicate product\nmatches in the union of all the per-product results."]
        #[serde(rename = "productGroupedResults", default)]
        pub product_grouped_results: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P5Beta1ProductSearchResultsGroupedResult>,
        >,
        #[doc = "List of results, one for each product match."]
        #[serde(rename = "results", default)]
        pub results: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P5Beta1ProductSearchResultsResult>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1ProductSearchResults {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1ProductSearchResultsGroupedResult {
        #[doc = "The bounding polygon around the product detected in the query image."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1BoundingPoly>,
        #[doc = "List of results, one for each product match."]
        #[serde(rename = "results", default)]
        pub results: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P5Beta1ProductSearchResultsResult>,
        >,
    }
    impl ::field_selector::FieldSelector
        for GoogleCloudVisionV1P5Beta1ProductSearchResultsGroupedResult
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1ProductSearchResultsResult {
        #[doc = "The resource name of the image from the product that is the closest match\nto the query."]
        #[serde(rename = "image", default)]
        pub image: ::std::option::Option<String>,
        #[doc = "The Product."]
        #[serde(rename = "product", default)]
        pub product: ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1Product>,
        #[doc = "A confidence level on the match, ranging from 0 (no confidence) to\n1 (full confidence)."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1ProductSearchResultsResult {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1Property {
        #[doc = "Name of the property."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Value of numeric properties."]
        #[serde(rename = "uint64Value", default)]
        #[serde(with = "crate::parsed_string")]
        pub uint_64_value: ::std::option::Option<u64>,
        #[doc = "Value of the property."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1Property {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1ReferenceImage {
        #[doc = "Bounding polygons around the areas of interest in the reference image.\nOptional. If this field is empty, the system will try to detect regions of\ninterest. At most 10 bounding polygons will be used.\n\nThe provided shape is converted into a non-rotated rectangle. Once\nconverted, the small edge of the rectangle must be greater than or equal\nto 300 pixels. The aspect ratio must be 1:4 or less (i.e. 1:3 is ok; 1:5\nis not)."]
        #[serde(rename = "boundingPolys", default)]
        pub bounding_polys:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P5Beta1BoundingPoly>>,
        #[doc = "The resource name of the reference image.\n\nFormat is:\n\n`projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID/referenceImages/IMAGE_ID`.\n\nThis field is ignored when creating a reference image."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The Google Cloud Storage URI of the reference image.\n\nThe URI must start with `gs://`.\n\nRequired."]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1ReferenceImage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1SafeSearchAnnotation {
        #[doc = "Represents the adult content likelihood for the image. Adult content may\ncontain elements such as nudity, pornographic images or cartoons, or\nsexual activities."]
        #[serde(rename = "adult", default)]
        pub adult: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P5Beta1SafeSearchAnnotationAdult,
        >,
        #[doc = "Confidence of adult_score. Range [0, 1]. 0 means not confident, 1 means\nvery confident."]
        #[serde(rename = "adultConfidence", default)]
        pub adult_confidence: ::std::option::Option<f32>,
        #[doc = "Likelihood that this is a medical image."]
        #[serde(rename = "medical", default)]
        pub medical: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P5Beta1SafeSearchAnnotationMedical,
        >,
        #[doc = "Confidence of medical_score. Range [0, 1]. 0 means not confident, 1 means\nvery confident."]
        #[serde(rename = "medicalConfidence", default)]
        pub medical_confidence: ::std::option::Option<f32>,
        #[doc = "Confidence of nsfw_score. Range [0, 1]. 0 means not confident, 1 means very\nconfident."]
        #[serde(rename = "nsfwConfidence", default)]
        pub nsfw_confidence: ::std::option::Option<f32>,
        #[doc = "Likelihood that the request image contains racy content. Racy content may\ninclude (but is not limited to) skimpy or sheer clothing, strategically\ncovered nudity, lewd or provocative poses, or close-ups of sensitive\nbody areas."]
        #[serde(rename = "racy", default)]
        pub racy: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P5Beta1SafeSearchAnnotationRacy,
        >,
        #[doc = "Confidence of racy_score. Range [0, 1]. 0 means not confident, 1 means very\nconfident."]
        #[serde(rename = "racyConfidence", default)]
        pub racy_confidence: ::std::option::Option<f32>,
        #[doc = "Spoof likelihood. The likelihood that an modification\nwas made to the image's canonical version to make it appear\nfunny or offensive."]
        #[serde(rename = "spoof", default)]
        pub spoof: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P5Beta1SafeSearchAnnotationSpoof,
        >,
        #[doc = "Confidence of spoof_score. Range [0, 1]. 0 means not confident, 1 means\nvery confident."]
        #[serde(rename = "spoofConfidence", default)]
        pub spoof_confidence: ::std::option::Option<f32>,
        #[doc = "Likelihood that this image contains violent content."]
        #[serde(rename = "violence", default)]
        pub violence: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P5Beta1SafeSearchAnnotationViolence,
        >,
        #[doc = "Confidence of violence_score. Range [0, 1]. 0 means not confident, 1 means\nvery confident."]
        #[serde(rename = "violenceConfidence", default)]
        pub violence_confidence: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1SafeSearchAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P5Beta1SafeSearchAnnotationAdult {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P5Beta1SafeSearchAnnotationAdult {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationAdult::Likely => "LIKELY",
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationAdult::Possible => "POSSIBLE",
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationAdult::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationAdult::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationAdult::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationAdult::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P5Beta1SafeSearchAnnotationAdult {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P5Beta1SafeSearchAnnotationAdult {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P5Beta1SafeSearchAnnotationAdult {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P5Beta1SafeSearchAnnotationAdult::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P5Beta1SafeSearchAnnotationAdult::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P5Beta1SafeSearchAnnotationAdult::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P5Beta1SafeSearchAnnotationAdult::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P5Beta1SafeSearchAnnotationAdult::VeryLikely,
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P5Beta1SafeSearchAnnotationAdult::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1SafeSearchAnnotationAdult {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P5Beta1SafeSearchAnnotationMedical {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P5Beta1SafeSearchAnnotationMedical {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationMedical::Likely => "LIKELY",
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationMedical::Possible => "POSSIBLE",
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationMedical::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationMedical::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationMedical::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationMedical::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P5Beta1SafeSearchAnnotationMedical {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P5Beta1SafeSearchAnnotationMedical {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P5Beta1SafeSearchAnnotationMedical {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P5Beta1SafeSearchAnnotationMedical::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P5Beta1SafeSearchAnnotationMedical::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P5Beta1SafeSearchAnnotationMedical::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P5Beta1SafeSearchAnnotationMedical::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P5Beta1SafeSearchAnnotationMedical::VeryLikely,
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P5Beta1SafeSearchAnnotationMedical::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1SafeSearchAnnotationMedical {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P5Beta1SafeSearchAnnotationRacy {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P5Beta1SafeSearchAnnotationRacy {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationRacy::Likely => "LIKELY",
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationRacy::Possible => "POSSIBLE",
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationRacy::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationRacy::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationRacy::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationRacy::VeryUnlikely => "VERY_UNLIKELY",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P5Beta1SafeSearchAnnotationRacy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P5Beta1SafeSearchAnnotationRacy {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P5Beta1SafeSearchAnnotationRacy {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P5Beta1SafeSearchAnnotationRacy::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P5Beta1SafeSearchAnnotationRacy::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P5Beta1SafeSearchAnnotationRacy::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P5Beta1SafeSearchAnnotationRacy::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P5Beta1SafeSearchAnnotationRacy::VeryLikely,
                "VERY_UNLIKELY" => GoogleCloudVisionV1P5Beta1SafeSearchAnnotationRacy::VeryUnlikely,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1SafeSearchAnnotationRacy {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P5Beta1SafeSearchAnnotationSpoof {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P5Beta1SafeSearchAnnotationSpoof {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationSpoof::Likely => "LIKELY",
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationSpoof::Possible => "POSSIBLE",
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationSpoof::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationSpoof::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationSpoof::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationSpoof::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P5Beta1SafeSearchAnnotationSpoof {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P5Beta1SafeSearchAnnotationSpoof {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P5Beta1SafeSearchAnnotationSpoof {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P5Beta1SafeSearchAnnotationSpoof::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P5Beta1SafeSearchAnnotationSpoof::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P5Beta1SafeSearchAnnotationSpoof::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P5Beta1SafeSearchAnnotationSpoof::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P5Beta1SafeSearchAnnotationSpoof::VeryLikely,
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P5Beta1SafeSearchAnnotationSpoof::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1SafeSearchAnnotationSpoof {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P5Beta1SafeSearchAnnotationViolence {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVisionV1P5Beta1SafeSearchAnnotationViolence {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationViolence::Likely => "LIKELY",
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationViolence::Possible => "POSSIBLE",
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationViolence::Unknown => "UNKNOWN",
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationViolence::Unlikely => "UNLIKELY",
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationViolence::VeryLikely => "VERY_LIKELY",
                GoogleCloudVisionV1P5Beta1SafeSearchAnnotationViolence::VeryUnlikely => {
                    "VERY_UNLIKELY"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P5Beta1SafeSearchAnnotationViolence {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P5Beta1SafeSearchAnnotationViolence {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P5Beta1SafeSearchAnnotationViolence {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => GoogleCloudVisionV1P5Beta1SafeSearchAnnotationViolence::Likely,
                "POSSIBLE" => GoogleCloudVisionV1P5Beta1SafeSearchAnnotationViolence::Possible,
                "UNKNOWN" => GoogleCloudVisionV1P5Beta1SafeSearchAnnotationViolence::Unknown,
                "UNLIKELY" => GoogleCloudVisionV1P5Beta1SafeSearchAnnotationViolence::Unlikely,
                "VERY_LIKELY" => GoogleCloudVisionV1P5Beta1SafeSearchAnnotationViolence::VeryLikely,
                "VERY_UNLIKELY" => {
                    GoogleCloudVisionV1P5Beta1SafeSearchAnnotationViolence::VeryUnlikely
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1SafeSearchAnnotationViolence {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1Symbol {
        #[doc = "The bounding box for the symbol.\nThe vertices are in the order of top-left, top-right, bottom-right,\nbottom-left. When a rotation of the bounding box is detected the rotation\nis represented as around the top-left corner as defined when the text is\nread in the 'natural' orientation.\nFor example:\n\n* when the text is horizontal it might look like:\n  0----1\n  |    |\n  3----2\n* when it's rotated 180 degrees around the top-left corner it becomes:\n  2----3\n  |    |\n  1----0\n  and the vertex order will still be (0, 1, 2, 3)."]
        #[serde(rename = "boundingBox", default)]
        pub bounding_box:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1BoundingPoly>,
        #[doc = "Confidence of the OCR results for the symbol. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Additional information detected for the symbol."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P5Beta1TextAnnotationTextProperty,
        >,
        #[doc = "The actual UTF-8 representation of the symbol."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1Symbol {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1Table {
        #[doc = "Body rows of the table"]
        #[serde(rename = "bodyRows", default)]
        pub body_rows:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P5Beta1TableTableRow>>,
        #[doc = "Header rows of the table"]
        #[serde(rename = "headerRows", default)]
        pub header_rows:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P5Beta1TableTableRow>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1Table {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1TableTableCell {
        #[doc = "How many columns this cell spans."]
        #[serde(rename = "colSpan", default)]
        pub col_span: ::std::option::Option<i32>,
        #[doc = "How many rows this cell spans."]
        #[serde(rename = "rowSpan", default)]
        pub row_span: ::std::option::Option<i32>,
        #[doc = "The merged text value of this cell, omitting any deeper structural\ninformation unlike `text_block`. This is useful for simple cells."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<String>,
        #[doc = "Text block for this cell which also contains the normalized bounding box\nfor the cell and deeper structures within a cell if present."]
        #[serde(rename = "textBlock", default)]
        pub text_block: ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1Block>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1TableTableCell {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1TableTableRow {
        #[doc = "Cells that make up this row."]
        #[serde(rename = "cells", default)]
        pub cells:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P5Beta1TableTableCell>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1TableTableRow {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1TextAnnotation {
        #[doc = "List of pages detected by OCR."]
        #[serde(rename = "pages", default)]
        pub pages: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P5Beta1Page>>,
        #[doc = "UTF-8 text detected on the pages."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1TextAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1TextAnnotationDetectedBreak {
        #[doc = "True if break prepends the element."]
        #[serde(rename = "isPrefix", default)]
        pub is_prefix: ::std::option::Option<bool>,
        #[doc = "Detected break type."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P5Beta1TextAnnotationDetectedBreakType,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1TextAnnotationDetectedBreak {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVisionV1P5Beta1TextAnnotationDetectedBreakType {
        #[doc = "Line-wrapping break."]
        EolSureSpace,
        #[doc = "End-line hyphen that is not present in text; does not co-occur with\n`SPACE`, `LEADER_SPACE`, or `LINE_BREAK`."]
        Hyphen,
        #[doc = "Line break that ends a paragraph."]
        LineBreak,
        #[doc = "Regular space."]
        Space,
        #[doc = "Sure space (very wide)."]
        SureSpace,
        #[doc = "Unknown break label type."]
        Unknown,
    }
    impl GoogleCloudVisionV1P5Beta1TextAnnotationDetectedBreakType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudVisionV1P5Beta1TextAnnotationDetectedBreakType::EolSureSpace => {
                    "EOL_SURE_SPACE"
                }
                GoogleCloudVisionV1P5Beta1TextAnnotationDetectedBreakType::Hyphen => "HYPHEN",
                GoogleCloudVisionV1P5Beta1TextAnnotationDetectedBreakType::LineBreak => {
                    "LINE_BREAK"
                }
                GoogleCloudVisionV1P5Beta1TextAnnotationDetectedBreakType::Space => "SPACE",
                GoogleCloudVisionV1P5Beta1TextAnnotationDetectedBreakType::SureSpace => {
                    "SURE_SPACE"
                }
                GoogleCloudVisionV1P5Beta1TextAnnotationDetectedBreakType::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudVisionV1P5Beta1TextAnnotationDetectedBreakType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVisionV1P5Beta1TextAnnotationDetectedBreakType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudVisionV1P5Beta1TextAnnotationDetectedBreakType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EOL_SURE_SPACE" => {
                    GoogleCloudVisionV1P5Beta1TextAnnotationDetectedBreakType::EolSureSpace
                }
                "HYPHEN" => GoogleCloudVisionV1P5Beta1TextAnnotationDetectedBreakType::Hyphen,
                "LINE_BREAK" => {
                    GoogleCloudVisionV1P5Beta1TextAnnotationDetectedBreakType::LineBreak
                }
                "SPACE" => GoogleCloudVisionV1P5Beta1TextAnnotationDetectedBreakType::Space,
                "SURE_SPACE" => {
                    GoogleCloudVisionV1P5Beta1TextAnnotationDetectedBreakType::SureSpace
                }
                "UNKNOWN" => GoogleCloudVisionV1P5Beta1TextAnnotationDetectedBreakType::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1TextAnnotationDetectedBreakType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1TextAnnotationDetectedLanguage {
        #[doc = "Confidence of detected language. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more\ninformation, see\nhttp://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1TextAnnotationDetectedLanguage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1TextAnnotationTextProperty {
        #[doc = "Detected start or end of a text segment."]
        #[serde(rename = "detectedBreak", default)]
        pub detected_break: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P5Beta1TextAnnotationDetectedBreak,
        >,
        #[doc = "A list of detected languages together with confidence."]
        #[serde(rename = "detectedLanguages", default)]
        pub detected_languages: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P5Beta1TextAnnotationDetectedLanguage>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1TextAnnotationTextProperty {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1Vertex {
        #[doc = "X coordinate."]
        #[serde(rename = "x", default)]
        pub x: ::std::option::Option<i32>,
        #[doc = "Y coordinate."]
        #[serde(rename = "y", default)]
        pub y: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1Vertex {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1WebDetection {
        #[doc = "The service's best guess as to the topic of the request image.\nInferred from similar images on the open web."]
        #[serde(rename = "bestGuessLabels", default)]
        pub best_guess_labels: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P5Beta1WebDetectionWebLabel>,
        >,
        #[doc = "Fully matching images from the Internet.\nCan include resized copies of the query image."]
        #[serde(rename = "fullMatchingImages", default)]
        pub full_matching_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P5Beta1WebDetectionWebImage>,
        >,
        #[doc = "Web pages containing the matching images from the Internet."]
        #[serde(rename = "pagesWithMatchingImages", default)]
        pub pages_with_matching_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P5Beta1WebDetectionWebPage>,
        >,
        #[doc = "Partial matching images from the Internet.\nThose images are similar enough to share some key-point features. For\nexample an original image will likely have partial matching for its crops."]
        #[serde(rename = "partialMatchingImages", default)]
        pub partial_matching_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P5Beta1WebDetectionWebImage>,
        >,
        #[doc = "The visually similar image results."]
        #[serde(rename = "visuallySimilarImages", default)]
        pub visually_similar_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P5Beta1WebDetectionWebImage>,
        >,
        #[doc = "Deduced entities from similar images on the Internet."]
        #[serde(rename = "webEntities", default)]
        pub web_entities: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P5Beta1WebDetectionWebEntity>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1WebDetection {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1WebDetectionWebEntity {
        #[doc = "Canonical description of the entity, in English."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Opaque entity ID."]
        #[serde(rename = "entityId", default)]
        pub entity_id: ::std::option::Option<String>,
        #[doc = "Overall relevancy score for the entity.\nNot normalized and not comparable across different image queries."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1WebDetectionWebEntity {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1WebDetectionWebImage {
        #[doc = "(Deprecated) Overall relevancy score for the image."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
        #[doc = "The result image URL."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1WebDetectionWebImage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1WebDetectionWebLabel {
        #[doc = "Label for extra metadata."]
        #[serde(rename = "label", default)]
        pub label: ::std::option::Option<String>,
        #[doc = "The BCP-47 language code for `label`, such as \"en-US\" or \"sr-Latn\".\nFor more information, see\nhttp://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1WebDetectionWebLabel {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1WebDetectionWebPage {
        #[doc = "Fully matching images on the page.\nCan include resized copies of the query image."]
        #[serde(rename = "fullMatchingImages", default)]
        pub full_matching_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P5Beta1WebDetectionWebImage>,
        >,
        #[doc = "Title for the web page, may contain HTML markups."]
        #[serde(rename = "pageTitle", default)]
        pub page_title: ::std::option::Option<String>,
        #[doc = "Partial matching images on the page.\nThose images are similar enough to share some key-point features. For\nexample an original image will likely have partial matching for its\ncrops."]
        #[serde(rename = "partialMatchingImages", default)]
        pub partial_matching_images: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVisionV1P5Beta1WebDetectionWebImage>,
        >,
        #[doc = "(Deprecated) Overall relevancy score for the web page."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
        #[doc = "The result web page URL."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1WebDetectionWebPage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVisionV1P5Beta1Word {
        #[doc = "The bounding box for the word.\nThe vertices are in the order of top-left, top-right, bottom-right,\nbottom-left. When a rotation of the bounding box is detected the rotation\nis represented as around the top-left corner as defined when the text is\nread in the 'natural' orientation.\nFor example:\n\n* when the text is horizontal it might look like:\n  0----1\n  |    |\n  3----2\n* when it's rotated 180 degrees around the top-left corner it becomes:\n  2----3\n  |    |\n  1----0\n  and the vertex order will still be (0, 1, 2, 3)."]
        #[serde(rename = "boundingBox", default)]
        pub bounding_box:
            ::std::option::Option<crate::schemas::GoogleCloudVisionV1P5Beta1BoundingPoly>,
        #[doc = "Confidence of the OCR results for the word. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "All UTF-8 text detected in this word. This field is by default not\nreturned unless specified in TextDetectionParams.word_filter."]
        #[serde(rename = "mergedText", default)]
        pub merged_text: ::std::option::Option<String>,
        #[doc = "Additional information detected for the word."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<
            crate::schemas::GoogleCloudVisionV1P5Beta1TextAnnotationTextProperty,
        >,
        #[doc = "List of symbols in the word.\nThe order of the symbols follows the natural reading order."]
        #[serde(rename = "symbols", default)]
        pub symbols: ::std::option::Option<Vec<crate::schemas::GoogleCloudVisionV1P5Beta1Symbol>>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudVisionV1P5Beta1Word {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GroupedResult {
        #[doc = "The bounding polygon around the product detected in the query image."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly: ::std::option::Option<crate::schemas::BoundingPoly>,
        #[doc = "List of results, one for each product match."]
        #[serde(rename = "results", default)]
        pub results: ::std::option::Option<Vec<crate::schemas::Result>>,
    }
    impl ::field_selector::FieldSelector for GroupedResult {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ImageAnnotationContext {
        #[doc = "If the file was a PDF or TIFF, this field gives the page number within\nthe file used to produce the image."]
        #[serde(rename = "pageNumber", default)]
        pub page_number: ::std::option::Option<i32>,
        #[doc = "The URI of the file used to produce the image."]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ImageAnnotationContext {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ImageProperties {
        #[doc = "If present, dominant colors completed successfully."]
        #[serde(rename = "dominantColors", default)]
        pub dominant_colors: ::std::option::Option<crate::schemas::DominantColorsAnnotation>,
    }
    impl ::field_selector::FieldSelector for ImageProperties {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ImportProductSetsResponse {
        #[doc = "The list of reference_images that are imported successfully."]
        #[serde(rename = "referenceImages", default)]
        pub reference_images: ::std::option::Option<Vec<crate::schemas::ReferenceImage>>,
        #[doc = "The rpc status for each ImportProductSet request, including both successes\nand errors.\n\nThe number of statuses here matches the number of lines in the csv file,\nand statuses[i] stores the success or failure status of processing the i-th\nline of the csv, starting from line 0."]
        #[serde(rename = "statuses", default)]
        pub statuses: ::std::option::Option<Vec<crate::schemas::Status>>,
    }
    impl ::field_selector::FieldSelector for ImportProductSetsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct InputConfig {
        #[doc = "File content, represented as a stream of bytes.\nNote: As with all `bytes` fields, protobuffers use a pure binary\nrepresentation, whereas JSON representations use base64.\n\nCurrently, this field only works for BatchAnnotateFiles requests. It does\nnot work for AsyncBatchAnnotateFiles requests."]
        #[serde(rename = "content", default)]
        pub content: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "The Google Cloud Storage location to read the input from."]
        #[serde(rename = "gcsSource", default)]
        pub gcs_source: ::std::option::Option<crate::schemas::GcsSource>,
        #[doc = "The type of the file. Currently only \"application/pdf\", \"image/tiff\" and\n\"image/gif\" are supported. Wildcards are not supported."]
        #[serde(rename = "mimeType", default)]
        pub mime_type: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for InputConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct KeyValue {
        #[doc = "The key of the label attached to the product. Cannot be empty and cannot\nexceed 128 bytes."]
        #[serde(rename = "key", default)]
        pub key: ::std::option::Option<String>,
        #[doc = "The value of the label attached to the product. Cannot be empty and\ncannot exceed 128 bytes."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for KeyValue {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Landmark {
        #[doc = "Face landmark position."]
        #[serde(rename = "position", default)]
        pub position: ::std::option::Option<crate::schemas::Position>,
        #[doc = "Face landmark type."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::LandmarkType>,
    }
    impl ::field_selector::FieldSelector for Landmark {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum LandmarkType {
        #[doc = "Chin gnathion."]
        ChinGnathion,
        #[doc = "Chin left gonion."]
        ChinLeftGonion,
        #[doc = "Chin right gonion."]
        ChinRightGonion,
        #[doc = "Forehead glabella."]
        ForeheadGlabella,
        #[doc = "Left ear tragion."]
        LeftEarTragion,
        #[doc = "Left eye."]
        LeftEye,
        #[doc = "Left eye, bottom boundary."]
        LeftEyeBottomBoundary,
        #[doc = "Left eye, left corner."]
        LeftEyeLeftCorner,
        #[doc = "Left eye pupil."]
        LeftEyePupil,
        #[doc = "Left eye, right corner."]
        LeftEyeRightCorner,
        #[doc = "Left eye, top boundary."]
        LeftEyeTopBoundary,
        #[doc = "Left eyebrow, upper midpoint."]
        LeftEyebrowUpperMidpoint,
        #[doc = "Left of left eyebrow."]
        LeftOfLeftEyebrow,
        #[doc = "Left of right eyebrow."]
        LeftOfRightEyebrow,
        #[doc = "Lower lip."]
        LowerLip,
        #[doc = "Midpoint between eyes."]
        MidpointBetweenEyes,
        #[doc = "Mouth center."]
        MouthCenter,
        #[doc = "Mouth left."]
        MouthLeft,
        #[doc = "Mouth right."]
        MouthRight,
        #[doc = "Nose, bottom center."]
        NoseBottomCenter,
        #[doc = "Nose, bottom left."]
        NoseBottomLeft,
        #[doc = "Nose, bottom right."]
        NoseBottomRight,
        #[doc = "Nose tip."]
        NoseTip,
        #[doc = "Right ear tragion."]
        RightEarTragion,
        #[doc = "Right eye."]
        RightEye,
        #[doc = "Right eye, bottom boundary."]
        RightEyeBottomBoundary,
        #[doc = "Right eye, left corner."]
        RightEyeLeftCorner,
        #[doc = "Right eye pupil."]
        RightEyePupil,
        #[doc = "Right eye, right corner."]
        RightEyeRightCorner,
        #[doc = "Right eye, top boundary."]
        RightEyeTopBoundary,
        #[doc = "Right eyebrow, upper midpoint."]
        RightEyebrowUpperMidpoint,
        #[doc = "Right of left eyebrow."]
        RightOfLeftEyebrow,
        #[doc = "Right of right eyebrow."]
        RightOfRightEyebrow,
        #[doc = "Unknown face landmark detected. Should not be filled."]
        UnknownLandmark,
        #[doc = "Upper lip."]
        UpperLip,
    }
    impl LandmarkType {
        pub fn as_str(self) -> &'static str {
            match self {
                LandmarkType::ChinGnathion => "CHIN_GNATHION",
                LandmarkType::ChinLeftGonion => "CHIN_LEFT_GONION",
                LandmarkType::ChinRightGonion => "CHIN_RIGHT_GONION",
                LandmarkType::ForeheadGlabella => "FOREHEAD_GLABELLA",
                LandmarkType::LeftEarTragion => "LEFT_EAR_TRAGION",
                LandmarkType::LeftEye => "LEFT_EYE",
                LandmarkType::LeftEyeBottomBoundary => "LEFT_EYE_BOTTOM_BOUNDARY",
                LandmarkType::LeftEyeLeftCorner => "LEFT_EYE_LEFT_CORNER",
                LandmarkType::LeftEyePupil => "LEFT_EYE_PUPIL",
                LandmarkType::LeftEyeRightCorner => "LEFT_EYE_RIGHT_CORNER",
                LandmarkType::LeftEyeTopBoundary => "LEFT_EYE_TOP_BOUNDARY",
                LandmarkType::LeftEyebrowUpperMidpoint => "LEFT_EYEBROW_UPPER_MIDPOINT",
                LandmarkType::LeftOfLeftEyebrow => "LEFT_OF_LEFT_EYEBROW",
                LandmarkType::LeftOfRightEyebrow => "LEFT_OF_RIGHT_EYEBROW",
                LandmarkType::LowerLip => "LOWER_LIP",
                LandmarkType::MidpointBetweenEyes => "MIDPOINT_BETWEEN_EYES",
                LandmarkType::MouthCenter => "MOUTH_CENTER",
                LandmarkType::MouthLeft => "MOUTH_LEFT",
                LandmarkType::MouthRight => "MOUTH_RIGHT",
                LandmarkType::NoseBottomCenter => "NOSE_BOTTOM_CENTER",
                LandmarkType::NoseBottomLeft => "NOSE_BOTTOM_LEFT",
                LandmarkType::NoseBottomRight => "NOSE_BOTTOM_RIGHT",
                LandmarkType::NoseTip => "NOSE_TIP",
                LandmarkType::RightEarTragion => "RIGHT_EAR_TRAGION",
                LandmarkType::RightEye => "RIGHT_EYE",
                LandmarkType::RightEyeBottomBoundary => "RIGHT_EYE_BOTTOM_BOUNDARY",
                LandmarkType::RightEyeLeftCorner => "RIGHT_EYE_LEFT_CORNER",
                LandmarkType::RightEyePupil => "RIGHT_EYE_PUPIL",
                LandmarkType::RightEyeRightCorner => "RIGHT_EYE_RIGHT_CORNER",
                LandmarkType::RightEyeTopBoundary => "RIGHT_EYE_TOP_BOUNDARY",
                LandmarkType::RightEyebrowUpperMidpoint => "RIGHT_EYEBROW_UPPER_MIDPOINT",
                LandmarkType::RightOfLeftEyebrow => "RIGHT_OF_LEFT_EYEBROW",
                LandmarkType::RightOfRightEyebrow => "RIGHT_OF_RIGHT_EYEBROW",
                LandmarkType::UnknownLandmark => "UNKNOWN_LANDMARK",
                LandmarkType::UpperLip => "UPPER_LIP",
            }
        }
    }
    impl ::std::fmt::Display for LandmarkType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for LandmarkType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LandmarkType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CHIN_GNATHION" => LandmarkType::ChinGnathion,
                "CHIN_LEFT_GONION" => LandmarkType::ChinLeftGonion,
                "CHIN_RIGHT_GONION" => LandmarkType::ChinRightGonion,
                "FOREHEAD_GLABELLA" => LandmarkType::ForeheadGlabella,
                "LEFT_EAR_TRAGION" => LandmarkType::LeftEarTragion,
                "LEFT_EYE" => LandmarkType::LeftEye,
                "LEFT_EYE_BOTTOM_BOUNDARY" => LandmarkType::LeftEyeBottomBoundary,
                "LEFT_EYE_LEFT_CORNER" => LandmarkType::LeftEyeLeftCorner,
                "LEFT_EYE_PUPIL" => LandmarkType::LeftEyePupil,
                "LEFT_EYE_RIGHT_CORNER" => LandmarkType::LeftEyeRightCorner,
                "LEFT_EYE_TOP_BOUNDARY" => LandmarkType::LeftEyeTopBoundary,
                "LEFT_EYEBROW_UPPER_MIDPOINT" => LandmarkType::LeftEyebrowUpperMidpoint,
                "LEFT_OF_LEFT_EYEBROW" => LandmarkType::LeftOfLeftEyebrow,
                "LEFT_OF_RIGHT_EYEBROW" => LandmarkType::LeftOfRightEyebrow,
                "LOWER_LIP" => LandmarkType::LowerLip,
                "MIDPOINT_BETWEEN_EYES" => LandmarkType::MidpointBetweenEyes,
                "MOUTH_CENTER" => LandmarkType::MouthCenter,
                "MOUTH_LEFT" => LandmarkType::MouthLeft,
                "MOUTH_RIGHT" => LandmarkType::MouthRight,
                "NOSE_BOTTOM_CENTER" => LandmarkType::NoseBottomCenter,
                "NOSE_BOTTOM_LEFT" => LandmarkType::NoseBottomLeft,
                "NOSE_BOTTOM_RIGHT" => LandmarkType::NoseBottomRight,
                "NOSE_TIP" => LandmarkType::NoseTip,
                "RIGHT_EAR_TRAGION" => LandmarkType::RightEarTragion,
                "RIGHT_EYE" => LandmarkType::RightEye,
                "RIGHT_EYE_BOTTOM_BOUNDARY" => LandmarkType::RightEyeBottomBoundary,
                "RIGHT_EYE_LEFT_CORNER" => LandmarkType::RightEyeLeftCorner,
                "RIGHT_EYE_PUPIL" => LandmarkType::RightEyePupil,
                "RIGHT_EYE_RIGHT_CORNER" => LandmarkType::RightEyeRightCorner,
                "RIGHT_EYE_TOP_BOUNDARY" => LandmarkType::RightEyeTopBoundary,
                "RIGHT_EYEBROW_UPPER_MIDPOINT" => LandmarkType::RightEyebrowUpperMidpoint,
                "RIGHT_OF_LEFT_EYEBROW" => LandmarkType::RightOfLeftEyebrow,
                "RIGHT_OF_RIGHT_EYEBROW" => LandmarkType::RightOfRightEyebrow,
                "UNKNOWN_LANDMARK" => LandmarkType::UnknownLandmark,
                "UPPER_LIP" => LandmarkType::UpperLip,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for LandmarkType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct LatLng {
        #[doc = "The latitude in degrees. It must be in the range [-90.0, +90.0]."]
        #[serde(rename = "latitude", default)]
        pub latitude: ::std::option::Option<f64>,
        #[doc = "The longitude in degrees. It must be in the range [-180.0, +180.0]."]
        #[serde(rename = "longitude", default)]
        pub longitude: ::std::option::Option<f64>,
    }
    impl ::field_selector::FieldSelector for LatLng {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct LocalizedObjectAnnotation {
        #[doc = "Image region to which this object belongs. This must be populated."]
        #[serde(rename = "boundingPoly", default)]
        pub bounding_poly: ::std::option::Option<crate::schemas::BoundingPoly>,
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more\ninformation, see\nhttp://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Object ID that should align with EntityAnnotation mid."]
        #[serde(rename = "mid", default)]
        pub mid: ::std::option::Option<String>,
        #[doc = "Object name, expressed in its `language_code` language."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Score of the result. Range [0, 1]."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for LocalizedObjectAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct LocationInfo {
        #[doc = "lat/long location coordinates."]
        #[serde(rename = "latLng", default)]
        pub lat_lng: ::std::option::Option<crate::schemas::LatLng>,
    }
    impl ::field_selector::FieldSelector for LocationInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct NormalizedVertex {
        #[doc = "X coordinate."]
        #[serde(rename = "x", default)]
        pub x: ::std::option::Option<f32>,
        #[doc = "Y coordinate."]
        #[serde(rename = "y", default)]
        pub y: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for NormalizedVertex {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Operation {
        #[doc = "If the value is `false`, it means the operation is still in progress.\nIf `true`, the operation is completed, and either `error` or `response` is\navailable."]
        #[serde(rename = "done", default)]
        pub done: ::std::option::Option<bool>,
        #[doc = "The error result of the operation in case of failure or cancellation."]
        #[serde(rename = "error", default)]
        pub error: ::std::option::Option<crate::schemas::Status>,
        #[doc = "Service-specific metadata associated with the operation.  It typically\ncontains progress information and common metadata such as create time.\nSome services might not provide such metadata.  Any method that returns a\nlong-running operation should document the metadata type, if any."]
        #[serde(rename = "metadata", default)]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The server-assigned name, which is only unique within the same service that\noriginally returns it. If you use the default HTTP mapping, the\n`name` should be a resource name ending with `operations/{unique_id}`."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The normal response of the operation in case of success.  If the original\nmethod returns no data on success, such as `Delete`, the response is\n`google.protobuf.Empty`.  If the original method is standard\n`Get`/`Create`/`Update`, the response should be the resource.  For other\nmethods, the response should have the type `XxxResponse`, where `Xxx`\nis the original method name.  For example, if the original method name\nis `TakeSnapshot()`, the inferred response type is\n`TakeSnapshotResponse`."]
        #[serde(rename = "response", default)]
        pub response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for Operation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OperationMetadata {
        #[doc = "The time when the batch request was received."]
        #[serde(rename = "createTime", default)]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Current state of the batch operation."]
        #[serde(rename = "state", default)]
        pub state: ::std::option::Option<crate::schemas::OperationMetadataState>,
        #[doc = "The time when the operation result was last updated."]
        #[serde(rename = "updateTime", default)]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for OperationMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OperationMetadataState {
        #[doc = "The batch processing was cancelled."]
        Cancelled,
        #[doc = "Request is received."]
        Created,
        #[doc = "The batch processing is done."]
        Done,
        #[doc = "Request is actively being processed."]
        Running,
        #[doc = "Invalid."]
        StateUnspecified,
    }
    impl OperationMetadataState {
        pub fn as_str(self) -> &'static str {
            match self {
                OperationMetadataState::Cancelled => "CANCELLED",
                OperationMetadataState::Created => "CREATED",
                OperationMetadataState::Done => "DONE",
                OperationMetadataState::Running => "RUNNING",
                OperationMetadataState::StateUnspecified => "STATE_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for OperationMetadataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OperationMetadataState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OperationMetadataState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => OperationMetadataState::Cancelled,
                "CREATED" => OperationMetadataState::Created,
                "DONE" => OperationMetadataState::Done,
                "RUNNING" => OperationMetadataState::Running,
                "STATE_UNSPECIFIED" => OperationMetadataState::StateUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for OperationMetadataState {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OutputConfig {
        #[doc = "The max number of response protos to put into each output JSON file on\nGoogle Cloud Storage.\nThe valid range is [1, 100]. If not specified, the default value is 20.\n\nFor example, for one pdf file with 100 pages, 100 response protos will\nbe generated. If `batch_size` = 20, then 5 json files each\ncontaining 20 response protos will be written under the prefix\n`gcs_destination`.`uri`.\n\nCurrently, batch_size only applies to GcsDestination, with potential future\nsupport for other output configurations."]
        #[serde(rename = "batchSize", default)]
        pub batch_size: ::std::option::Option<i32>,
        #[doc = "The Google Cloud Storage location to write the output(s) to."]
        #[serde(rename = "gcsDestination", default)]
        pub gcs_destination: ::std::option::Option<crate::schemas::GcsDestination>,
    }
    impl ::field_selector::FieldSelector for OutputConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Page {
        #[doc = "List of blocks of text, images etc on this page."]
        #[serde(rename = "blocks", default)]
        pub blocks: ::std::option::Option<Vec<crate::schemas::Block>>,
        #[doc = "Confidence of the OCR results on the page. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Page height. For PDFs the unit is points. For images (including\nTIFFs) the unit is pixels."]
        #[serde(rename = "height", default)]
        pub height: ::std::option::Option<i32>,
        #[doc = "Additional information detected on the page."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<crate::schemas::TextProperty>,
        #[doc = "Page width. For PDFs the unit is points. For images (including\nTIFFs) the unit is pixels."]
        #[serde(rename = "width", default)]
        pub width: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for Page {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Paragraph {
        #[doc = "The bounding box for the paragraph.\nThe vertices are in the order of top-left, top-right, bottom-right,\nbottom-left. When a rotation of the bounding box is detected the rotation\nis represented as around the top-left corner as defined when the text is\nread in the 'natural' orientation.\nFor example:\n\n* when the text is horizontal it might look like:\n  0----1\n  |    |\n  3----2\n* when it's rotated 180 degrees around the top-left corner it becomes:\n  2----3\n  |    |\n  1----0\n  and the vertex order will still be (0, 1, 2, 3)."]
        #[serde(rename = "boundingBox", default)]
        pub bounding_box: ::std::option::Option<crate::schemas::BoundingPoly>,
        #[doc = "Confidence of the OCR results for the paragraph. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Additional information detected for the paragraph."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<crate::schemas::TextProperty>,
        #[doc = "List of all words in this paragraph."]
        #[serde(rename = "words", default)]
        pub words: ::std::option::Option<Vec<crate::schemas::Word>>,
    }
    impl ::field_selector::FieldSelector for Paragraph {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Position {
        #[doc = "X coordinate."]
        #[serde(rename = "x", default)]
        pub x: ::std::option::Option<f32>,
        #[doc = "Y coordinate."]
        #[serde(rename = "y", default)]
        pub y: ::std::option::Option<f32>,
        #[doc = "Z coordinate (or depth)."]
        #[serde(rename = "z", default)]
        pub z: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for Position {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Product {
        #[doc = "User-provided metadata to be stored with this product. Must be at most 4096\ncharacters long."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "The user-provided name for this Product. Must not be empty. Must be at most\n4096 characters long."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The resource name of the product.\n\nFormat is:\n`projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`.\n\nThis field is ignored when creating a product."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The category for the product identified by the reference image. This should\nbe either \"homegoods-v2\", \"apparel-v2\", or \"toys-v2\". The legacy categories\n\"homegoods\", \"apparel\", and \"toys\" are still supported, but these should\nnot be used for new products.\n\nThis field is immutable."]
        #[serde(rename = "productCategory", default)]
        pub product_category: ::std::option::Option<String>,
        #[doc = "Key-value pairs that can be attached to a product. At query time,\nconstraints can be specified based on the product_labels.\n\nNote that integer values can be provided as strings, e.g. \"1199\". Only\nstrings with integer values can match a range-based restriction which is\nto be supported soon.\n\nMultiple values can be assigned to the same key. One product may have up to\n500 product_labels.\n\nNotice that the total number of distinct product_labels over all products\nin one ProductSet cannot exceed 1M, otherwise the product search pipeline\nwill refuse to work for that ProductSet."]
        #[serde(rename = "productLabels", default)]
        pub product_labels: ::std::option::Option<Vec<crate::schemas::KeyValue>>,
    }
    impl ::field_selector::FieldSelector for Product {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ProductSearchResults {
        #[doc = "Timestamp of the index which provided these results. Products added to the\nproduct set and products removed from the product set after this time are\nnot reflected in the current results."]
        #[serde(rename = "indexTime", default)]
        pub index_time: ::std::option::Option<String>,
        #[doc = "List of results grouped by products detected in the query image. Each entry\ncorresponds to one bounding polygon in the query image, and contains the\nmatching products specific to that region. There may be duplicate product\nmatches in the union of all the per-product results."]
        #[serde(rename = "productGroupedResults", default)]
        pub product_grouped_results: ::std::option::Option<Vec<crate::schemas::GroupedResult>>,
        #[doc = "List of results, one for each product match."]
        #[serde(rename = "results", default)]
        pub results: ::std::option::Option<Vec<crate::schemas::Result>>,
    }
    impl ::field_selector::FieldSelector for ProductSearchResults {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Property {
        #[doc = "Name of the property."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Value of numeric properties."]
        #[serde(rename = "uint64Value", default)]
        #[serde(with = "crate::parsed_string")]
        pub uint_64_value: ::std::option::Option<u64>,
        #[doc = "Value of the property."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Property {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ReferenceImage {
        #[doc = "Bounding polygons around the areas of interest in the reference image.\nOptional. If this field is empty, the system will try to detect regions of\ninterest. At most 10 bounding polygons will be used.\n\nThe provided shape is converted into a non-rotated rectangle. Once\nconverted, the small edge of the rectangle must be greater than or equal\nto 300 pixels. The aspect ratio must be 1:4 or less (i.e. 1:3 is ok; 1:5\nis not)."]
        #[serde(rename = "boundingPolys", default)]
        pub bounding_polys: ::std::option::Option<Vec<crate::schemas::BoundingPoly>>,
        #[doc = "The resource name of the reference image.\n\nFormat is:\n\n`projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID/referenceImages/IMAGE_ID`.\n\nThis field is ignored when creating a reference image."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The Google Cloud Storage URI of the reference image.\n\nThe URI must start with `gs://`.\n\nRequired."]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ReferenceImage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Result {
        #[doc = "The resource name of the image from the product that is the closest match\nto the query."]
        #[serde(rename = "image", default)]
        pub image: ::std::option::Option<String>,
        #[doc = "The Product."]
        #[serde(rename = "product", default)]
        pub product: ::std::option::Option<crate::schemas::Product>,
        #[doc = "A confidence level on the match, ranging from 0 (no confidence) to\n1 (full confidence)."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for Result {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SafeSearchAnnotation {
        #[doc = "Represents the adult content likelihood for the image. Adult content may\ncontain elements such as nudity, pornographic images or cartoons, or\nsexual activities."]
        #[serde(rename = "adult", default)]
        pub adult: ::std::option::Option<crate::schemas::SafeSearchAnnotationAdult>,
        #[doc = "Confidence of adult_score. Range [0, 1]. 0 means not confident, 1 means\nvery confident."]
        #[serde(rename = "adultConfidence", default)]
        pub adult_confidence: ::std::option::Option<f32>,
        #[doc = "Likelihood that this is a medical image."]
        #[serde(rename = "medical", default)]
        pub medical: ::std::option::Option<crate::schemas::SafeSearchAnnotationMedical>,
        #[doc = "Confidence of medical_score. Range [0, 1]. 0 means not confident, 1 means\nvery confident."]
        #[serde(rename = "medicalConfidence", default)]
        pub medical_confidence: ::std::option::Option<f32>,
        #[doc = "Confidence of nsfw_score. Range [0, 1]. 0 means not confident, 1 means very\nconfident."]
        #[serde(rename = "nsfwConfidence", default)]
        pub nsfw_confidence: ::std::option::Option<f32>,
        #[doc = "Likelihood that the request image contains racy content. Racy content may\ninclude (but is not limited to) skimpy or sheer clothing, strategically\ncovered nudity, lewd or provocative poses, or close-ups of sensitive\nbody areas."]
        #[serde(rename = "racy", default)]
        pub racy: ::std::option::Option<crate::schemas::SafeSearchAnnotationRacy>,
        #[doc = "Confidence of racy_score. Range [0, 1]. 0 means not confident, 1 means very\nconfident."]
        #[serde(rename = "racyConfidence", default)]
        pub racy_confidence: ::std::option::Option<f32>,
        #[doc = "Spoof likelihood. The likelihood that an modification\nwas made to the image's canonical version to make it appear\nfunny or offensive."]
        #[serde(rename = "spoof", default)]
        pub spoof: ::std::option::Option<crate::schemas::SafeSearchAnnotationSpoof>,
        #[doc = "Confidence of spoof_score. Range [0, 1]. 0 means not confident, 1 means\nvery confident."]
        #[serde(rename = "spoofConfidence", default)]
        pub spoof_confidence: ::std::option::Option<f32>,
        #[doc = "Likelihood that this image contains violent content."]
        #[serde(rename = "violence", default)]
        pub violence: ::std::option::Option<crate::schemas::SafeSearchAnnotationViolence>,
        #[doc = "Confidence of violence_score. Range [0, 1]. 0 means not confident, 1 means\nvery confident."]
        #[serde(rename = "violenceConfidence", default)]
        pub violence_confidence: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for SafeSearchAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SafeSearchAnnotationAdult {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl SafeSearchAnnotationAdult {
        pub fn as_str(self) -> &'static str {
            match self {
                SafeSearchAnnotationAdult::Likely => "LIKELY",
                SafeSearchAnnotationAdult::Possible => "POSSIBLE",
                SafeSearchAnnotationAdult::Unknown => "UNKNOWN",
                SafeSearchAnnotationAdult::Unlikely => "UNLIKELY",
                SafeSearchAnnotationAdult::VeryLikely => "VERY_LIKELY",
                SafeSearchAnnotationAdult::VeryUnlikely => "VERY_UNLIKELY",
            }
        }
    }
    impl ::std::fmt::Display for SafeSearchAnnotationAdult {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SafeSearchAnnotationAdult {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SafeSearchAnnotationAdult {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => SafeSearchAnnotationAdult::Likely,
                "POSSIBLE" => SafeSearchAnnotationAdult::Possible,
                "UNKNOWN" => SafeSearchAnnotationAdult::Unknown,
                "UNLIKELY" => SafeSearchAnnotationAdult::Unlikely,
                "VERY_LIKELY" => SafeSearchAnnotationAdult::VeryLikely,
                "VERY_UNLIKELY" => SafeSearchAnnotationAdult::VeryUnlikely,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for SafeSearchAnnotationAdult {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SafeSearchAnnotationMedical {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl SafeSearchAnnotationMedical {
        pub fn as_str(self) -> &'static str {
            match self {
                SafeSearchAnnotationMedical::Likely => "LIKELY",
                SafeSearchAnnotationMedical::Possible => "POSSIBLE",
                SafeSearchAnnotationMedical::Unknown => "UNKNOWN",
                SafeSearchAnnotationMedical::Unlikely => "UNLIKELY",
                SafeSearchAnnotationMedical::VeryLikely => "VERY_LIKELY",
                SafeSearchAnnotationMedical::VeryUnlikely => "VERY_UNLIKELY",
            }
        }
    }
    impl ::std::fmt::Display for SafeSearchAnnotationMedical {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SafeSearchAnnotationMedical {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SafeSearchAnnotationMedical {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => SafeSearchAnnotationMedical::Likely,
                "POSSIBLE" => SafeSearchAnnotationMedical::Possible,
                "UNKNOWN" => SafeSearchAnnotationMedical::Unknown,
                "UNLIKELY" => SafeSearchAnnotationMedical::Unlikely,
                "VERY_LIKELY" => SafeSearchAnnotationMedical::VeryLikely,
                "VERY_UNLIKELY" => SafeSearchAnnotationMedical::VeryUnlikely,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for SafeSearchAnnotationMedical {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SafeSearchAnnotationRacy {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl SafeSearchAnnotationRacy {
        pub fn as_str(self) -> &'static str {
            match self {
                SafeSearchAnnotationRacy::Likely => "LIKELY",
                SafeSearchAnnotationRacy::Possible => "POSSIBLE",
                SafeSearchAnnotationRacy::Unknown => "UNKNOWN",
                SafeSearchAnnotationRacy::Unlikely => "UNLIKELY",
                SafeSearchAnnotationRacy::VeryLikely => "VERY_LIKELY",
                SafeSearchAnnotationRacy::VeryUnlikely => "VERY_UNLIKELY",
            }
        }
    }
    impl ::std::fmt::Display for SafeSearchAnnotationRacy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SafeSearchAnnotationRacy {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SafeSearchAnnotationRacy {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => SafeSearchAnnotationRacy::Likely,
                "POSSIBLE" => SafeSearchAnnotationRacy::Possible,
                "UNKNOWN" => SafeSearchAnnotationRacy::Unknown,
                "UNLIKELY" => SafeSearchAnnotationRacy::Unlikely,
                "VERY_LIKELY" => SafeSearchAnnotationRacy::VeryLikely,
                "VERY_UNLIKELY" => SafeSearchAnnotationRacy::VeryUnlikely,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for SafeSearchAnnotationRacy {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SafeSearchAnnotationSpoof {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl SafeSearchAnnotationSpoof {
        pub fn as_str(self) -> &'static str {
            match self {
                SafeSearchAnnotationSpoof::Likely => "LIKELY",
                SafeSearchAnnotationSpoof::Possible => "POSSIBLE",
                SafeSearchAnnotationSpoof::Unknown => "UNKNOWN",
                SafeSearchAnnotationSpoof::Unlikely => "UNLIKELY",
                SafeSearchAnnotationSpoof::VeryLikely => "VERY_LIKELY",
                SafeSearchAnnotationSpoof::VeryUnlikely => "VERY_UNLIKELY",
            }
        }
    }
    impl ::std::fmt::Display for SafeSearchAnnotationSpoof {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SafeSearchAnnotationSpoof {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SafeSearchAnnotationSpoof {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => SafeSearchAnnotationSpoof::Likely,
                "POSSIBLE" => SafeSearchAnnotationSpoof::Possible,
                "UNKNOWN" => SafeSearchAnnotationSpoof::Unknown,
                "UNLIKELY" => SafeSearchAnnotationSpoof::Unlikely,
                "VERY_LIKELY" => SafeSearchAnnotationSpoof::VeryLikely,
                "VERY_UNLIKELY" => SafeSearchAnnotationSpoof::VeryUnlikely,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for SafeSearchAnnotationSpoof {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SafeSearchAnnotationViolence {
        #[doc = "It is likely."]
        Likely,
        #[doc = "It is possible."]
        Possible,
        #[doc = "Unknown likelihood."]
        Unknown,
        #[doc = "It is unlikely."]
        Unlikely,
        #[doc = "It is very likely."]
        VeryLikely,
        #[doc = "It is very unlikely."]
        VeryUnlikely,
    }
    impl SafeSearchAnnotationViolence {
        pub fn as_str(self) -> &'static str {
            match self {
                SafeSearchAnnotationViolence::Likely => "LIKELY",
                SafeSearchAnnotationViolence::Possible => "POSSIBLE",
                SafeSearchAnnotationViolence::Unknown => "UNKNOWN",
                SafeSearchAnnotationViolence::Unlikely => "UNLIKELY",
                SafeSearchAnnotationViolence::VeryLikely => "VERY_LIKELY",
                SafeSearchAnnotationViolence::VeryUnlikely => "VERY_UNLIKELY",
            }
        }
    }
    impl ::std::fmt::Display for SafeSearchAnnotationViolence {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SafeSearchAnnotationViolence {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SafeSearchAnnotationViolence {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LIKELY" => SafeSearchAnnotationViolence::Likely,
                "POSSIBLE" => SafeSearchAnnotationViolence::Possible,
                "UNKNOWN" => SafeSearchAnnotationViolence::Unknown,
                "UNLIKELY" => SafeSearchAnnotationViolence::Unlikely,
                "VERY_LIKELY" => SafeSearchAnnotationViolence::VeryLikely,
                "VERY_UNLIKELY" => SafeSearchAnnotationViolence::VeryUnlikely,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for SafeSearchAnnotationViolence {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Status {
        #[doc = "The status code, which should be an enum value of google.rpc.Code."]
        #[serde(rename = "code", default)]
        pub code: ::std::option::Option<i32>,
        #[doc = "A list of messages that carry the error details.  There is a common set of\nmessage types for APIs to use."]
        #[serde(rename = "details", default)]
        pub details:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "A developer-facing error message, which should be in English. Any\nuser-facing error message should be localized and sent in the\ngoogle.rpc.Status.details field, or localized by the client."]
        #[serde(rename = "message", default)]
        pub message: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Status {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Symbol {
        #[doc = "The bounding box for the symbol.\nThe vertices are in the order of top-left, top-right, bottom-right,\nbottom-left. When a rotation of the bounding box is detected the rotation\nis represented as around the top-left corner as defined when the text is\nread in the 'natural' orientation.\nFor example:\n\n* when the text is horizontal it might look like:\n  0----1\n  |    |\n  3----2\n* when it's rotated 180 degrees around the top-left corner it becomes:\n  2----3\n  |    |\n  1----0\n  and the vertex order will still be (0, 1, 2, 3)."]
        #[serde(rename = "boundingBox", default)]
        pub bounding_box: ::std::option::Option<crate::schemas::BoundingPoly>,
        #[doc = "Confidence of the OCR results for the symbol. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Additional information detected for the symbol."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<crate::schemas::TextProperty>,
        #[doc = "The actual UTF-8 representation of the symbol."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Symbol {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TextAnnotation {
        #[doc = "List of pages detected by OCR."]
        #[serde(rename = "pages", default)]
        pub pages: ::std::option::Option<Vec<crate::schemas::Page>>,
        #[doc = "UTF-8 text detected on the pages."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for TextAnnotation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TextProperty {
        #[doc = "Detected start or end of a text segment."]
        #[serde(rename = "detectedBreak", default)]
        pub detected_break: ::std::option::Option<crate::schemas::DetectedBreak>,
        #[doc = "A list of detected languages together with confidence."]
        #[serde(rename = "detectedLanguages", default)]
        pub detected_languages: ::std::option::Option<Vec<crate::schemas::DetectedLanguage>>,
    }
    impl ::field_selector::FieldSelector for TextProperty {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Vertex {
        #[doc = "X coordinate."]
        #[serde(rename = "x", default)]
        pub x: ::std::option::Option<i32>,
        #[doc = "Y coordinate."]
        #[serde(rename = "y", default)]
        pub y: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for Vertex {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct WebDetection {
        #[doc = "The service's best guess as to the topic of the request image.\nInferred from similar images on the open web."]
        #[serde(rename = "bestGuessLabels", default)]
        pub best_guess_labels: ::std::option::Option<Vec<crate::schemas::WebLabel>>,
        #[doc = "Fully matching images from the Internet.\nCan include resized copies of the query image."]
        #[serde(rename = "fullMatchingImages", default)]
        pub full_matching_images: ::std::option::Option<Vec<crate::schemas::WebImage>>,
        #[doc = "Web pages containing the matching images from the Internet."]
        #[serde(rename = "pagesWithMatchingImages", default)]
        pub pages_with_matching_images: ::std::option::Option<Vec<crate::schemas::WebPage>>,
        #[doc = "Partial matching images from the Internet.\nThose images are similar enough to share some key-point features. For\nexample an original image will likely have partial matching for its crops."]
        #[serde(rename = "partialMatchingImages", default)]
        pub partial_matching_images: ::std::option::Option<Vec<crate::schemas::WebImage>>,
        #[doc = "The visually similar image results."]
        #[serde(rename = "visuallySimilarImages", default)]
        pub visually_similar_images: ::std::option::Option<Vec<crate::schemas::WebImage>>,
        #[doc = "Deduced entities from similar images on the Internet."]
        #[serde(rename = "webEntities", default)]
        pub web_entities: ::std::option::Option<Vec<crate::schemas::WebEntity>>,
    }
    impl ::field_selector::FieldSelector for WebDetection {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct WebEntity {
        #[doc = "Canonical description of the entity, in English."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Opaque entity ID."]
        #[serde(rename = "entityId", default)]
        pub entity_id: ::std::option::Option<String>,
        #[doc = "Overall relevancy score for the entity.\nNot normalized and not comparable across different image queries."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
    }
    impl ::field_selector::FieldSelector for WebEntity {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct WebImage {
        #[doc = "(Deprecated) Overall relevancy score for the image."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
        #[doc = "The result image URL."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for WebImage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct WebLabel {
        #[doc = "Label for extra metadata."]
        #[serde(rename = "label", default)]
        pub label: ::std::option::Option<String>,
        #[doc = "The BCP-47 language code for `label`, such as \"en-US\" or \"sr-Latn\".\nFor more information, see\nhttp://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for WebLabel {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct WebPage {
        #[doc = "Fully matching images on the page.\nCan include resized copies of the query image."]
        #[serde(rename = "fullMatchingImages", default)]
        pub full_matching_images: ::std::option::Option<Vec<crate::schemas::WebImage>>,
        #[doc = "Title for the web page, may contain HTML markups."]
        #[serde(rename = "pageTitle", default)]
        pub page_title: ::std::option::Option<String>,
        #[doc = "Partial matching images on the page.\nThose images are similar enough to share some key-point features. For\nexample an original image will likely have partial matching for its\ncrops."]
        #[serde(rename = "partialMatchingImages", default)]
        pub partial_matching_images: ::std::option::Option<Vec<crate::schemas::WebImage>>,
        #[doc = "(Deprecated) Overall relevancy score for the web page."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
        #[doc = "The result web page URL."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for WebPage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Word {
        #[doc = "The bounding box for the word.\nThe vertices are in the order of top-left, top-right, bottom-right,\nbottom-left. When a rotation of the bounding box is detected the rotation\nis represented as around the top-left corner as defined when the text is\nread in the 'natural' orientation.\nFor example:\n\n* when the text is horizontal it might look like:\n  0----1\n  |    |\n  3----2\n* when it's rotated 180 degrees around the top-left corner it becomes:\n  2----3\n  |    |\n  1----0\n  and the vertex order will still be (0, 1, 2, 3)."]
        #[serde(rename = "boundingBox", default)]
        pub bounding_box: ::std::option::Option<crate::schemas::BoundingPoly>,
        #[doc = "Confidence of the OCR results for the word. Range [0, 1]."]
        #[serde(rename = "confidence", default)]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Additional information detected for the word."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<crate::schemas::TextProperty>,
        #[doc = "List of symbols in the word.\nThe order of the symbols follows the natural reading order."]
        #[serde(rename = "symbols", default)]
        pub symbols: ::std::option::Option<Vec<crate::schemas::Symbol>>,
    }
    impl ::field_selector::FieldSelector for Word {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
}
pub mod params {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Alt {
        #[doc = "Responses with Content-Type of application/json"]
        Json,
        #[doc = "Media download with context-dependent Content-Type"]
        Media,
        #[doc = "Responses with Content-Type of application/x-protobuf"]
        Proto,
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
                Alt::Media => "media",
                Alt::Proto => "proto",
            }
        }
    }
    impl ::std::fmt::Display for Alt {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Alt {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Alt {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "json" => Alt::Json,
                "media" => Alt::Media,
                "proto" => Alt::Proto,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for Alt {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Xgafv {
        #[doc = "v1 error format"]
        _1,
        #[doc = "v2 error format"]
        _2,
    }
    impl Xgafv {
        pub fn as_str(self) -> &'static str {
            match self {
                Xgafv::_1 => "1",
                Xgafv::_2 => "2",
            }
        }
    }
    impl ::std::fmt::Display for Xgafv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Xgafv {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Xgafv {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "1" => Xgafv::_1,
                "2" => Xgafv::_2,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for Xgafv {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
}
pub struct Client<A> {
    reqwest: ::reqwest::Client,
    auth: ::std::sync::Mutex<A>,
}
impl<A: yup_oauth2::GetToken> Client<A> {
    pub fn new(auth: A) -> Self {
        Client {
            reqwest: ::reqwest::Client::builder().timeout(None).build().unwrap(),
            auth: ::std::sync::Mutex::new(auth),
        }
    }
    #[doc = "Actions that can be performed on the files resource"]
    pub fn files(&self) -> crate::resources::files::FilesActions<A> {
        crate::resources::files::FilesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the images resource"]
    pub fn images(&self) -> crate::resources::images::ImagesActions<A> {
        crate::resources::images::ImagesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the projects resource"]
    pub fn projects(&self) -> crate::resources::projects::ProjectsActions<A> {
        crate::resources::projects::ProjectsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod resources {
    pub mod files {
        pub mod params {}
        pub struct FilesActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> FilesActions<'a, A> {
            #[doc = "Service that performs image detection and annotation for a batch of files.\nNow only \"application/pdf\", \"image/tiff\" and \"image/gif\" are supported.\n\nThis service will extract at most 5 (customers can specify which 5 in\nAnnotateFileRequest.pages) frames (gif) or pages (pdf or tiff) from each\nfile provided and perform detection and annotation for each image\nextracted."]
            pub fn annotate(
                &self,
                request: crate::schemas::GoogleCloudVisionV1P1Beta1BatchAnnotateFilesRequest,
            ) -> AnnotateRequestBuilder<A> {
                AnnotateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
            #[doc = "Run asynchronous image detection and annotation for a list of generic\nfiles, such as PDF files, which may contain multiple pages and multiple\nimages per page. Progress and results can be retrieved through the\n`google.longrunning.Operations` interface.\n`Operation.metadata` contains `OperationMetadata` (metadata).\n`Operation.response` contains `AsyncBatchAnnotateFilesResponse` (results)."]
            pub fn async_batch_annotate(
                &self,
                request: crate::schemas::GoogleCloudVisionV1P1Beta1AsyncBatchAnnotateFilesRequest,
            ) -> AsyncBatchAnnotateRequestBuilder<A> {
                AsyncBatchAnnotateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct AnnotateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::GoogleCloudVisionV1P1Beta1BatchAnnotateFilesRequest,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> AnnotateRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<
                crate::schemas::GoogleCloudVisionV1P1Beta1BatchAnnotateFilesResponse,
                Box<dyn ::std::error::Error>,
            > {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<
                crate::schemas::GoogleCloudVisionV1P1Beta1BatchAnnotateFilesResponse,
                Box<dyn ::std::error::Error>,
            > {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://vision.googleapis.com/".to_owned();
                output.push_str("v1p1beta1/files:annotate");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct AsyncBatchAnnotateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::GoogleCloudVisionV1P1Beta1AsyncBatchAnnotateFilesRequest,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> AsyncBatchAnnotateRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>> {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://vision.googleapis.com/".to_owned();
                output.push_str("v1p1beta1/files:asyncBatchAnnotate");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
                req
            }
        }
    }
    pub mod images {
        pub mod params {}
        pub struct ImagesActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> ImagesActions<'a, A> {
            #[doc = "Run image detection and annotation for a batch of images."]
            pub fn annotate(
                &self,
                request: crate::schemas::GoogleCloudVisionV1P1Beta1BatchAnnotateImagesRequest,
            ) -> AnnotateRequestBuilder<A> {
                AnnotateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
            #[doc = "Run asynchronous image detection and annotation for a list of images.\n\nProgress and results can be retrieved through the\n`google.longrunning.Operations` interface.\n`Operation.metadata` contains `OperationMetadata` (metadata).\n`Operation.response` contains `AsyncBatchAnnotateImagesResponse` (results).\n\nThis service will write image annotation outputs to json files in customer\nGCS bucket, each json file containing BatchAnnotateImagesResponse proto."]
            pub fn async_batch_annotate(
                &self,
                request: crate::schemas::GoogleCloudVisionV1P1Beta1AsyncBatchAnnotateImagesRequest,
            ) -> AsyncBatchAnnotateRequestBuilder<A> {
                AsyncBatchAnnotateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct AnnotateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::GoogleCloudVisionV1P1Beta1BatchAnnotateImagesRequest,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> AnnotateRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<
                crate::schemas::GoogleCloudVisionV1P1Beta1BatchAnnotateImagesResponse,
                Box<dyn ::std::error::Error>,
            > {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<
                crate::schemas::GoogleCloudVisionV1P1Beta1BatchAnnotateImagesResponse,
                Box<dyn ::std::error::Error>,
            > {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://vision.googleapis.com/".to_owned();
                output.push_str("v1p1beta1/images:annotate");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct AsyncBatchAnnotateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::GoogleCloudVisionV1P1Beta1AsyncBatchAnnotateImagesRequest,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> AsyncBatchAnnotateRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>> {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://vision.googleapis.com/".to_owned();
                output.push_str("v1p1beta1/images:asyncBatchAnnotate");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
                req
            }
        }
    }
    pub mod projects {
        pub mod params {}
        pub struct ProjectsActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> ProjectsActions<'a, A> {
            #[doc = "Actions that can be performed on the files resource"]
            pub fn files(&self) -> crate::resources::projects::files::FilesActions<A> {
                crate::resources::projects::files::FilesActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
            #[doc = "Actions that can be performed on the images resource"]
            pub fn images(&self) -> crate::resources::projects::images::ImagesActions<A> {
                crate::resources::projects::images::ImagesActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
            #[doc = "Actions that can be performed on the locations resource"]
            pub fn locations(&self) -> crate::resources::projects::locations::LocationsActions<A> {
                crate::resources::projects::locations::LocationsActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
        }
        pub mod files {
            pub mod params {}
            pub struct FilesActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> FilesActions<'a, A> {
                #[doc = "Service that performs image detection and annotation for a batch of files.\nNow only \"application/pdf\", \"image/tiff\" and \"image/gif\" are supported.\n\nThis service will extract at most 5 (customers can specify which 5 in\nAnnotateFileRequest.pages) frames (gif) or pages (pdf or tiff) from each\nfile provided and perform detection and annotation for each image\nextracted."]
                pub fn annotate(
                    &self,
                    request: crate::schemas::GoogleCloudVisionV1P1Beta1BatchAnnotateFilesRequest,
                    parent: impl Into<String>,
                ) -> AnnotateRequestBuilder<A> {
                    AnnotateRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        parent: parent.into(),
                    }
                }
                #[doc = "Run asynchronous image detection and annotation for a list of generic\nfiles, such as PDF files, which may contain multiple pages and multiple\nimages per page. Progress and results can be retrieved through the\n`google.longrunning.Operations` interface.\n`Operation.metadata` contains `OperationMetadata` (metadata).\n`Operation.response` contains `AsyncBatchAnnotateFilesResponse` (results)."]
                pub fn async_batch_annotate(
                    &self,
                    request : crate :: schemas :: GoogleCloudVisionV1P1Beta1AsyncBatchAnnotateFilesRequest,
                    parent: impl Into<String>,
                ) -> AsyncBatchAnnotateRequestBuilder<A> {
                    AsyncBatchAnnotateRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        parent: parent.into(),
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct AnnotateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::GoogleCloudVisionV1P1Beta1BatchAnnotateFilesRequest,
                parent: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> AnnotateRequestBuilder<'a, A> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    let fields = T::field_selector();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_standard(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudVisionV1P1Beta1BatchAnnotateFilesResponse,
                    Box<dyn ::std::error::Error>,
                > {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudVisionV1P1Beta1BatchAnnotateFilesResponse,
                    Box<dyn ::std::error::Error>,
                > {
                    self.execute_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path());
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://vision.googleapis.com/".to_owned();
                    output.push_str("v1p1beta1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/files:annotate");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct AsyncBatchAnnotateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::GoogleCloudVisionV1P1Beta1AsyncBatchAnnotateFilesRequest,
                parent: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> AsyncBatchAnnotateRequestBuilder<'a, A> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    let fields = T::field_selector();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_standard(
                    self,
                ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path());
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://vision.googleapis.com/".to_owned();
                    output.push_str("v1p1beta1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/files:asyncBatchAnnotate");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
        }
        pub mod images {
            pub mod params {}
            pub struct ImagesActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> ImagesActions<'a, A> {
                #[doc = "Run image detection and annotation for a batch of images."]
                pub fn annotate(
                    &self,
                    request: crate::schemas::GoogleCloudVisionV1P1Beta1BatchAnnotateImagesRequest,
                    parent: impl Into<String>,
                ) -> AnnotateRequestBuilder<A> {
                    AnnotateRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        parent: parent.into(),
                    }
                }
                #[doc = "Run asynchronous image detection and annotation for a list of images.\n\nProgress and results can be retrieved through the\n`google.longrunning.Operations` interface.\n`Operation.metadata` contains `OperationMetadata` (metadata).\n`Operation.response` contains `AsyncBatchAnnotateImagesResponse` (results).\n\nThis service will write image annotation outputs to json files in customer\nGCS bucket, each json file containing BatchAnnotateImagesResponse proto."]
                pub fn async_batch_annotate(
                    &self,
                    request : crate :: schemas :: GoogleCloudVisionV1P1Beta1AsyncBatchAnnotateImagesRequest,
                    parent: impl Into<String>,
                ) -> AsyncBatchAnnotateRequestBuilder<A> {
                    AsyncBatchAnnotateRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        parent: parent.into(),
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct AnnotateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::GoogleCloudVisionV1P1Beta1BatchAnnotateImagesRequest,
                parent: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> AnnotateRequestBuilder<'a, A> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    let fields = T::field_selector();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_standard(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudVisionV1P1Beta1BatchAnnotateImagesResponse,
                    Box<dyn ::std::error::Error>,
                > {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudVisionV1P1Beta1BatchAnnotateImagesResponse,
                    Box<dyn ::std::error::Error>,
                > {
                    self.execute_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path());
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://vision.googleapis.com/".to_owned();
                    output.push_str("v1p1beta1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/images:annotate");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct AsyncBatchAnnotateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::GoogleCloudVisionV1P1Beta1AsyncBatchAnnotateImagesRequest,
                parent: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> AsyncBatchAnnotateRequestBuilder<'a, A> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    let fields = T::field_selector();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_standard(
                    self,
                ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path());
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://vision.googleapis.com/".to_owned();
                    output.push_str("v1p1beta1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/images:asyncBatchAnnotate");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
        }
        pub mod locations {
            pub mod params {}
            pub struct LocationsActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> LocationsActions<'a, A> {
                #[doc = "Actions that can be performed on the files resource"]
                pub fn files(
                    &self,
                ) -> crate::resources::projects::locations::files::FilesActions<A> {
                    crate::resources::projects::locations::files::FilesActions {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                    }
                }
                #[doc = "Actions that can be performed on the images resource"]
                pub fn images(
                    &self,
                ) -> crate::resources::projects::locations::images::ImagesActions<A>
                {
                    crate::resources::projects::locations::images::ImagesActions {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                    }
                }
            }
            pub mod files {
                pub mod params {}
                pub struct FilesActions<'a, A> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> FilesActions<'a, A> {
                    #[doc = "Service that performs image detection and annotation for a batch of files.\nNow only \"application/pdf\", \"image/tiff\" and \"image/gif\" are supported.\n\nThis service will extract at most 5 (customers can specify which 5 in\nAnnotateFileRequest.pages) frames (gif) or pages (pdf or tiff) from each\nfile provided and perform detection and annotation for each image\nextracted."]
                    pub fn annotate(
                        &self,
                        request : crate :: schemas :: GoogleCloudVisionV1P1Beta1BatchAnnotateFilesRequest,
                        parent: impl Into<String>,
                    ) -> AnnotateRequestBuilder<A> {
                        AnnotateRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                        }
                    }
                    #[doc = "Run asynchronous image detection and annotation for a list of generic\nfiles, such as PDF files, which may contain multiple pages and multiple\nimages per page. Progress and results can be retrieved through the\n`google.longrunning.Operations` interface.\n`Operation.metadata` contains `OperationMetadata` (metadata).\n`Operation.response` contains `AsyncBatchAnnotateFilesResponse` (results)."]
                    pub fn async_batch_annotate(
                        &self,
                        request : crate :: schemas :: GoogleCloudVisionV1P1Beta1AsyncBatchAnnotateFilesRequest,
                        parent: impl Into<String>,
                    ) -> AsyncBatchAnnotateRequestBuilder<A> {
                        AsyncBatchAnnotateRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct AnnotateRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::GoogleCloudVisionV1P1Beta1BatchAnnotateFilesRequest,
                    parent: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> AnnotateRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        let fields = T::field_selector();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_standard(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleCloudVisionV1P1Beta1BatchAnnotateFilesResponse,
                        Box<dyn ::std::error::Error>,
                    > {
                        self.execute_fields::<_, &str>(None)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_debug(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleCloudVisionV1P1Beta1BatchAnnotateFilesResponse,
                        Box<dyn ::std::error::Error>,
                    > {
                        self.execute_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://vision.googleapis.com/".to_owned();
                        output.push_str("v1p1beta1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/files:annotate");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let fut =
                            auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                        let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                        let token = runtime.block_on(fut).unwrap().access_token;
                        let req = req.bearer_auth(&token);
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct AsyncBatchAnnotateRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request:
                        crate::schemas::GoogleCloudVisionV1P1Beta1AsyncBatchAnnotateFilesRequest,
                    parent: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> AsyncBatchAnnotateRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        let fields = T::field_selector();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_standard(
                        self,
                    ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>>
                    {
                        self.execute_fields::<_, &str>(None)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>>
                    {
                        self.execute_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://vision.googleapis.com/".to_owned();
                        output.push_str("v1p1beta1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/files:asyncBatchAnnotate");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let fut =
                            auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                        let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                        let token = runtime.block_on(fut).unwrap().access_token;
                        let req = req.bearer_auth(&token);
                        req
                    }
                }
            }
            pub mod images {
                pub mod params {}
                pub struct ImagesActions<'a, A> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> ImagesActions<'a, A> {
                    #[doc = "Run image detection and annotation for a batch of images."]
                    pub fn annotate(
                        &self,
                        request : crate :: schemas :: GoogleCloudVisionV1P1Beta1BatchAnnotateImagesRequest,
                        parent: impl Into<String>,
                    ) -> AnnotateRequestBuilder<A> {
                        AnnotateRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                        }
                    }
                    #[doc = "Run asynchronous image detection and annotation for a list of images.\n\nProgress and results can be retrieved through the\n`google.longrunning.Operations` interface.\n`Operation.metadata` contains `OperationMetadata` (metadata).\n`Operation.response` contains `AsyncBatchAnnotateImagesResponse` (results).\n\nThis service will write image annotation outputs to json files in customer\nGCS bucket, each json file containing BatchAnnotateImagesResponse proto."]
                    pub fn async_batch_annotate(
                        &self,
                        request : crate :: schemas :: GoogleCloudVisionV1P1Beta1AsyncBatchAnnotateImagesRequest,
                        parent: impl Into<String>,
                    ) -> AsyncBatchAnnotateRequestBuilder<A> {
                        AsyncBatchAnnotateRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct AnnotateRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::GoogleCloudVisionV1P1Beta1BatchAnnotateImagesRequest,
                    parent: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> AnnotateRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        let fields = T::field_selector();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_standard(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleCloudVisionV1P1Beta1BatchAnnotateImagesResponse,
                        Box<dyn ::std::error::Error>,
                    > {
                        self.execute_fields::<_, &str>(None)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_debug(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleCloudVisionV1P1Beta1BatchAnnotateImagesResponse,
                        Box<dyn ::std::error::Error>,
                    > {
                        self.execute_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://vision.googleapis.com/".to_owned();
                        output.push_str("v1p1beta1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/images:annotate");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let fut =
                            auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                        let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                        let token = runtime.block_on(fut).unwrap().access_token;
                        let req = req.bearer_auth(&token);
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct AsyncBatchAnnotateRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request:
                        crate::schemas::GoogleCloudVisionV1P1Beta1AsyncBatchAnnotateImagesRequest,
                    parent: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> AsyncBatchAnnotateRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        let fields = T::field_selector();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_standard(
                        self,
                    ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>>
                    {
                        self.execute_fields::<_, &str>(None)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>>
                    {
                        self.execute_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path());
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://vision.googleapis.com/".to_owned();
                        output.push_str("v1p1beta1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/images:asyncBatchAnnotate");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let fut =
                            auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                        let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                        let token = runtime.block_on(fut).unwrap().access_token;
                        let req = req.bearer_auth(&token);
                        req
                    }
                }
            }
        }
    }
}
#[allow(dead_code)]
const SIMPLE: &::percent_encoding::AsciiSet = &::percent_encoding::NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

#[allow(dead_code)]
const RESERVED: &::percent_encoding::AsciiSet = &SIMPLE
    .remove(b'%')
    .remove(b':')
    .remove(b'/')
    .remove(b'?')
    .remove(b'#')
    .remove(b'[')
    .remove(b']')
    .remove(b'@')
    .remove(b'!')
    .remove(b'$')
    .remove(b'&')
    .remove(b'\'')
    .remove(b'(')
    .remove(b')')
    .remove(b'*')
    .remove(b'+')
    .remove(b',')
    .remove(b';')
    .remove(b'=');
#[allow(dead_code)]
mod multipart {
    pub(crate) struct RelatedMultiPart {
        parts: Vec<Part>,
        boundary: String,
    }

    impl RelatedMultiPart {
        pub(crate) fn new() -> Self {
            RelatedMultiPart {
                parts: Vec::new(),
                boundary: ::textnonce::TextNonce::sized(68).unwrap().0,
            }
        }

        pub(crate) fn new_part(&mut self, part: Part) {
            self.parts.push(part);
        }

        pub(crate) fn boundary(&self) -> &str {
            &self.boundary
        }

        pub(crate) fn into_reader(self) -> RelatedMultiPartReader {
            let boundary_marker = boundary_marker(&self.boundary);
            RelatedMultiPartReader {
                state: RelatedMultiPartReaderState::WriteBoundary {
                    start: 0,
                    boundary: format!("{}\r\n", &boundary_marker),
                },
                boundary: boundary_marker,
                next_body: None,
                parts: self.parts.into_iter(),
            }
        }
    }

    pub(crate) struct Part {
        content_type: ::mime::Mime,
        body: Box<dyn ::std::io::Read + Send>,
    }

    impl Part {
        pub(crate) fn new(
            content_type: ::mime::Mime,
            body: Box<dyn ::std::io::Read + Send>,
        ) -> Part {
            Part { content_type, body }
        }
    }

    pub(crate) struct RelatedMultiPartReader {
        state: RelatedMultiPartReaderState,
        boundary: String,
        next_body: Option<Box<dyn ::std::io::Read + Send>>,
        parts: std::vec::IntoIter<Part>,
    }

    enum RelatedMultiPartReaderState {
        WriteBoundary {
            start: usize,
            boundary: String,
        },
        WriteContentType {
            start: usize,
            content_type: Vec<u8>,
        },
        WriteBody {
            body: Box<dyn ::std::io::Read + Send>,
        },
    }

    impl ::std::io::Read for RelatedMultiPartReader {
        fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
            use RelatedMultiPartReaderState::*;
            let mut bytes_written: usize = 0;
            loop {
                let rem_buf = &mut buf[bytes_written..];
                match &mut self.state {
                    WriteBoundary { start, boundary } => {
                        let bytes_to_copy = std::cmp::min(boundary.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&boundary.as_bytes()[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == boundary.len() {
                            let next_part = match self.parts.next() {
                                None => break,
                                Some(part) => part,
                            };
                            self.next_body = Some(next_part.body);
                            self.state = WriteContentType {
                                start: 0,
                                content_type: format!(
                                    "Content-Type: {}\r\n\r\n",
                                    next_part.content_type
                                )
                                .into_bytes(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteContentType {
                        start,
                        content_type,
                    } => {
                        let bytes_to_copy =
                            std::cmp::min(content_type.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&content_type[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == content_type.len() {
                            self.state = WriteBody {
                                body: self.next_body.take().unwrap(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteBody { body } => {
                        let written = body.read(rem_buf)?;
                        bytes_written += written;
                        if written == 0 {
                            self.state = WriteBoundary {
                                start: 0,
                                boundary: format!("\r\n{}\r\n", &self.boundary),
                            };
                        } else {
                            break;
                        }
                    }
                }
            }
            Ok(bytes_written)
        }
    }

    fn boundary_marker(boundary: &str) -> String {
        let mut marker = String::with_capacity(boundary.len() + 2);
        marker.push_str("--");
        marker.push_str(boundary);
        marker
    }
}
// A serde helper module that can be used with the `with` attribute
// to deserialize any string to a FromStr type and serialize any
// Display type to a String. Google API's encode i64, u64 values as
// strings.
#[allow(dead_code)]
mod parsed_string {
    pub fn serialize<T, S>(
        value: &Option<T>,
        serializer: S,
    ) -> ::std::result::Result<S::Ok, S::Error>
    where
        T: ::std::fmt::Display,
        S: ::serde::Serializer,
    {
        use ::serde::Serialize;
        value.as_ref().map(|x| x.to_string()).serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> ::std::result::Result<Option<T>, D::Error>
    where
        T: ::std::str::FromStr,
        T::Err: ::std::fmt::Display,
        D: ::serde::de::Deserializer<'de>,
    {
        use ::serde::Deserialize;
        match Option::<String>::deserialize(deserializer)? {
            Some(x) => Ok(Some(x.parse().map_err(::serde::de::Error::custom)?)),
            None => Ok(None),
        }
    }
}
// Bytes in google apis are represented as urlsafe base64 encoded strings.
// This defines a Bytes type that is a simple wrapper around a Vec<u8> used
// internally to handle byte fields in google apis.
pub mod bytes {
    use radix64::URL_SAFE as BASE64_CFG;

    #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct Bytes(Vec<u8>);

    impl ::std::convert::From<Vec<u8>> for Bytes {
        fn from(x: Vec<u8>) -> Bytes {
            Bytes(x)
        }
    }

    impl ::std::fmt::Display for Bytes {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> ::std::fmt::Result {
            ::radix64::Display::new(BASE64_CFG, &self.0).fmt(f)
        }
    }

    impl ::serde::Serialize for Bytes {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            let encoded = BASE64_CFG.encode(&self.0);
            encoded.serialize(serializer)
        }
    }

    impl<'de> ::serde::Deserialize<'de> for Bytes {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Bytes, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            let encoded = String::deserialize(deserializer)?;
            let decoded = BASE64_CFG
                .decode(&encoded)
                .map_err(|_| ::serde::de::Error::custom("invalid base64 input"))?;
            Ok(Bytes(decoded))
        }
    }
}
