use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SearchResultItem {
    pub id: String,
    pub entity_type: String, // "project", "code", "document", "resume", "skill"
    pub title: String,
    pub snippet: String,
    pub score: f64,
}

#[derive(Debug, Clone)]
pub struct RankedItem {
    pub id: String,
    pub entity_type: String,
    pub title: String,
    pub snippet: String,
    pub rank: usize, // 1-indexed
    pub raw_score: f64,
}

pub fn compute_rrf_score(keyword_rank: Option<usize>, vector_rank: Option<usize>, k: usize) -> f64 {
    let mut score = 0.0;
    if let Some(rank) = keyword_rank {
        score += 1.0 / (k + rank) as f64;
    }
    if let Some(rank) = vector_rank {
        score += 1.0 / (k + rank) as f64;
    }
    score
}

pub struct HybridSearchEngine {
    pub rrf_k: usize,
}

impl Default for HybridSearchEngine {
    fn default() -> Self {
        Self { rrf_k: 60 }
    }
}

impl HybridSearchEngine {
    pub fn new(rrf_k: usize) -> Self {
        Self { rrf_k }
    }

    pub fn fuse_results(
        &self,
        keyword_results: &[RankedItem],
        vector_results: &[RankedItem],
        filter_entity: Option<&str>,
        limit: usize,
    ) -> Vec<SearchResultItem> {
        let mut map: HashMap<String, (Option<usize>, Option<usize>, RankedItem)> = HashMap::new();

        for item in keyword_results {
            map.entry(item.id.clone())
                .or_insert((None, None, item.clone()))
                .0 = Some(item.rank);
        }

        for item in vector_results {
            let entry = map
                .entry(item.id.clone())
                .or_insert((None, None, item.clone()));
            entry.1 = Some(item.rank);
        }

        let mut fused: Vec<SearchResultItem> = map
            .into_iter()
            .filter_map(|(id, (kw_rank, vec_rank, item))| {
                if let Some(entity) = filter_entity {
                    if item.entity_type != entity {
                        return None;
                    }
                }

                let score = compute_rrf_score(kw_rank, vec_rank, self.rrf_k);
                Some(SearchResultItem {
                    id,
                    entity_type: item.entity_type,
                    title: item.title,
                    snippet: item.snippet,
                    score,
                })
            })
            .collect();

        fused.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        fused.truncate(limit);

        fused
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rrf_score_calculation() {
        let score = compute_rrf_score(Some(1), Some(1), 60);
        assert!((score - (2.0 / 61.0)).abs() < 1e-6);
    }

    #[test]
    fn test_hybrid_search_fusion() {
        let engine = HybridSearchEngine::default();

        let kw = vec![RankedItem {
            id: "proj-1".to_string(),
            entity_type: "project".to_string(),
            title: "DevResume API".to_string(),
            snippet: "Rust backend".to_string(),
            rank: 1,
            raw_score: 0.9,
        }];

        let vec_res = vec![RankedItem {
            id: "proj-1".to_string(),
            entity_type: "project".to_string(),
            title: "DevResume API".to_string(),
            snippet: "Rust backend".to_string(),
            rank: 1,
            raw_score: 0.95,
        }];

        let results = engine.fuse_results(&kw, &vec_res, None, 10);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "proj-1");
    }
}
