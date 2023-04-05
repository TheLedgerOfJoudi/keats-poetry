use kd_tree::KdPoint;
use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType, SentenceEmbeddingsModel,
};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Library {
    pub poems: Vec<Poem>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Poem {
    pub title: String,
    pub lines: String,
}

#[derive(Debug)]
pub struct EmbeddedPoem {
    pub title: String,
    pub lines: String,
    pub embeddings: [f32; 384],
}

impl EmbeddedPoem {
    fn topic(embeddings: [f32; 384]) -> Self {
        Self {
            title: "None".to_string(),
            lines: "None".to_string(),
            embeddings: embeddings,
        }
    }
}

impl KdPoint for EmbeddedPoem {
    type Scalar = f32;
    type Dim = typenum::U2; // 2 dimensional tree.
    fn at(&self, k: usize) -> f32 {
        self.embeddings[k]
    }
}

impl Poem {
    fn to_embedded(self, embeddings: [f32; 384]) -> EmbeddedPoem {
        EmbeddedPoem {
            title: self.title,
            lines: self.lines,
            embeddings: embeddings,
        }
    }
}

// convenient to convert a slice to a fixed size array
fn to_array(barry: &[f32]) -> [f32; 384] {
    barry.try_into().expect("slice with incorrect length")
}

pub fn search_for_poem(title: &str) -> anyhow::Result<()> {
    let model:SentenceEmbeddingsModel = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
        .create_model()?;
    let json = fs::read_to_string("db/poems.json")?;
    let library: Library = serde_json::from_str(&json)?;
    let mut embedded_poems:Vec<EmbeddedPoem> = Vec::new();
    for poem in library.poems.clone() {
        let embeddings= model.encode(&[poem.clone().lines])?;
        let embedding = to_array(embeddings[0].as_slice());
        embedded_poems.push(poem.to_embedded(embedding));
    }
    let kdtree = kd_tree::KdSlice::sort_by(&mut embedded_poems, |item1, item2, k| {
        item1.embeddings[k]
            .partial_cmp(&item2.embeddings[k])
            .unwrap()
    });
    let title_embeddings = model.encode(&[title])?;
    let title_embeddings = to_array(title_embeddings[0].as_slice());
    let title_topic = EmbeddedPoem::topic(title_embeddings);
    let nearests = kdtree.nearests(&title_topic, 10);
    for nearest in nearests {
        println!("nearest: {:?}", nearest.item.title);
        println!("distance: {:?}", nearest.squared_distance);
    }
    Ok(())
}
