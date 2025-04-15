use serde::{ser::SerializeStruct, Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum NodeType {
    #[default]
    Paragraph,
    Root,
    Heading(HeadingLevel),
    /// Tracks position
    ListItem(u8),
    /// A List can be ordered or unordered.
    List(bool),
}

#[derive(Debug, Default)]
pub struct Node {
    /// Unique identifier
    pub id: uuid::Uuid,
    pub node_type: NodeType,
    pub contents: Vec<TextNode>,
    pub children: Option<Vec<Node>>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            ..Default::default()
        }
    }
}

impl Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Node", 4)?;
        state.serialize_field("id", &self.id.to_string())?;
        state.serialize_field("node_type", &self.node_type)?;
        state.serialize_field("children", &self.children)?;
        state.serialize_field("contents", &self.contents)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Node {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct NodeHelper {
            id: String,
            node_type: NodeType,
            contents: Vec<TextNode>,
            children: Option<Vec<Node>>,
        }

        let node = NodeHelper::deserialize(deserializer)?;

        match uuid::Uuid::parse_str(&node.id) {
            Ok(id) => {
                let de_node = Node {
                    id,
                    node_type: node.node_type,
                    contents: node.contents,
                    children: node.children,
                };

                Ok(de_node)
            }
            Err(why) => {
                let msg = format!("parsing error failed with message {why}");
                let err: D::Error = serde::de::Error::custom(msg);

                Err(err)
            }
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TextFormat {
    pub size: u8,
    pub underline: bool,
    pub strong: bool,
    pub strikethrough: bool,
    pub emphasis: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TextNode {
    pub content: String,
    pub format: TextFormat,
}

#[derive(Debug, Default)]
pub enum HeadingLevel {
    #[default]
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
}

impl Serialize for HeadingLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            HeadingLevel::Level1 => serializer.serialize_i8(1),
            HeadingLevel::Level2 => serializer.serialize_i8(2),
            HeadingLevel::Level3 => serializer.serialize_i8(3),
            HeadingLevel::Level4 => serializer.serialize_i8(4),
            HeadingLevel::Level5 => serializer.serialize_i8(5),
            HeadingLevel::Level6 => serializer.serialize_i8(6),
        }
    }
}

impl<'de> Deserialize<'de> for HeadingLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;

        let level = match value {
            0 | 1 => HeadingLevel::Level1,
            2 => HeadingLevel::Level2,
            3 => HeadingLevel::Level3,
            4 => HeadingLevel::Level4,
            5 => HeadingLevel::Level5,
            _ => HeadingLevel::Level6,
        };

        Ok(level)
    }
}

#[derive(Debug, Default)]
pub struct ListItem {
    pub content: Vec<TextNode>,
    pub position: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    root: Node,
}

impl Document {
    pub fn root_node(self) -> Node {
        self.root
    }

    pub fn new() -> Self {
        Self {
            root: Node::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub books: Vec<Chapter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chapter {
    pub num: u8,
    pub title: Option<String>,
    pub pages: Document,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub origin: Setting,
    pub description: Document,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
    pub description: Document,
}
