use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::error::VerboseError;
use nom::Parser;

type OrthographicalVariant = &'static [&'static str];

pub struct OrthographicalVariantAdapter {
    pub variant_list: Vec<OrthographicalVariant>,
}

impl OrthographicalVariantAdapter {
    pub fn apply(self, input: &str, region_name: &str) -> Option<(String, String)> {
        let mut filtered_variant_list: Vec<OrthographicalVariant> = vec![];
        // 必要なパターンのみを選別する
        for variant in self.variant_list.clone() {
            if variant.iter().any(|character| input.contains(character)) {
                filtered_variant_list.push(variant);
            }
        }
        if filtered_variant_list.is_empty() {
            return None;
        }

        // マッチ候補を容れておくためのVector
        let mut candidates: Vec<String> = vec![region_name.to_string()];
        // パターンを一つづつ検証していく
        for variant in filtered_variant_list {
            let mut semi_candidates: Vec<String> = vec![];
            // variantから順列を作成
            // ["ケ", "ヶ", "が"] -> (ケ, ヶ), (ケ, が), (ヶ, ケ), (ヶ, が), (が, ケ), (が, ヶ)
            for permutation in variant.iter().permutations(2) {
                for candidate in &candidates {
                    // マッチ候補の中でパターンに引っかかるものがあれば文字を置き換えてマッチを試す
                    if candidate.contains(permutation[0]) {
                        let edited_region_name = candidate.replace(permutation[0], permutation[1]);
                        match tag::<&str, &str, VerboseError<&str>>(&edited_region_name)
                            .parse(input)
                        {
                            // マッチすれば早期リターン
                            Ok((rest, _)) => {
                                return Some((rest.to_string(), region_name.to_string()))
                            }
                            // マッチしなければsemi_candidatesに置き換え後の文字列をpush
                            Err(_) => {
                                semi_candidates.push(edited_region_name.clone());
                            }
                        };
                    }
                }
            }
            candidates = semi_candidates;
            candidates.push(region_name.to_string())
        }
        None
    }
}
