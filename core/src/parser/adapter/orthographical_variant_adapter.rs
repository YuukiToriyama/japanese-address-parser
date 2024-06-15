use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::error::VerboseError;
use nom::Parser;

pub type Variant = &'static [&'static str];

pub trait OrthographicalVariants {
    const の: Variant;
    const ツ: Variant;
    const ケ: Variant;
    const 薮: Variant;
    const 崎: Variant;
    const 檜: Variant;
    const 龍: Variant;
    const 竈: Variant;
    const 嶋: Variant;
    const 舘: Variant;
    const 脊: Variant;
    const 渕: Variant;
    const 己: Variant;
    const 槇: Variant;
    const 治: Variant;
    const 佛: Variant;
    const 澤: Variant;
    const 塚: Variant;
    const 恵: Variant;
    const 穂: Variant;
    const 梼: Variant;
    const 葛: Variant;
}

impl OrthographicalVariants for Variant {
    const の: Variant = &["の", "ノ"];
    const ツ: Variant = &["ツ", "ッ"];
    const ケ: Variant = &["ケ", "ヶ", "が", "ガ"];
    const 薮: Variant = &["薮", "藪", "籔"];
    const 崎: Variant = &["崎", "﨑"];
    const 檜: Variant = &["桧", "檜"];
    const 龍: Variant = &["龍", "竜"];
    const 竈: Variant = &["竈", "釜"];
    const 嶋: Variant = &["嶋", "島"];
    const 舘: Variant = &["舘", "館"];
    const 脊: Variant = &["脊", "背"];
    const 渕: Variant = &["渕", "淵"];
    const 己: Variant = &["己", "巳"];
    const 槇: Variant = &["槇", "槙"];
    const 治: Variant = &["治", "冶"];
    const 佛: Variant = &["佛", "仏"];
    const 澤: Variant = &["澤", "沢"];
    const 塚: Variant = &["塚", "塚"];
    const 恵: Variant = &["恵", "惠"];
    const 穂: Variant = &["穂", "穗"];
    const 梼: Variant = &["梼", "檮"];
    const 葛: Variant = &["葛󠄀", "葛"];
}

pub struct OrthographicalVariantAdapter {
    pub variant_list: Vec<Variant>,
}

impl OrthographicalVariantAdapter {
    pub fn apply(self, input: &str, region_name: &str) -> Option<(String, String)> {
        let mut filtered_variant_list: Vec<Variant> = vec![];
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
                        if let Ok((rest, _)) =
                            tag::<&str, &str, VerboseError<&str>>(&edited_region_name).parse(input)
                        {
                            // マッチすれば早期リターン
                            return Some((rest.to_string(), region_name.to_string()));
                        } else {
                            // マッチしなければsemi_candidatesに置き換え後の文字列をpush
                            semi_candidates.push(edited_region_name.clone());
                        };
                    }
                }
            }
            candidates = semi_candidates;
            candidates.push(region_name.to_string());
        }
        None
    }
}
