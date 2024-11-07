#[derive(Clone)]
pub enum OrthographicalVariant {
    の,
    ツ,
    ケ,
    薮,
    崎,
    檜,
    龍,
    竈,
    嶋,
    舘,
    脊,
    渕,
    己,
    槇,
    治,
    佛,
    澤,
    塚,
    恵,
    穂,
    梼,
    蛍,
    與,
    瀧,
    籠,
    濱,
    祗,
    曾,
}

impl OrthographicalVariant {
    fn value(&self) -> &[char] {
        match self {
            OrthographicalVariant::の => &['の', 'ノ', '之'],
            OrthographicalVariant::ツ => &['ツ', 'ッ'],
            OrthographicalVariant::ケ => &['ケ', 'ヶ', 'が', 'ガ'],
            OrthographicalVariant::薮 => &['薮', '藪', '籔'],
            OrthographicalVariant::崎 => &['崎', '﨑'],
            OrthographicalVariant::檜 => &['桧', '檜'],
            OrthographicalVariant::龍 => &['龍', '竜'],
            OrthographicalVariant::竈 => &['竈', '竃', '釜'],
            OrthographicalVariant::嶋 => &['嶋', '島'],
            OrthographicalVariant::舘 => &['舘', '館'],
            OrthographicalVariant::脊 => &['脊', '背'],
            OrthographicalVariant::渕 => &['渕', '淵'],
            OrthographicalVariant::己 => &['己', '巳'],
            OrthographicalVariant::槇 => &['槇', '槙'],
            OrthographicalVariant::治 => &['治', '冶'],
            OrthographicalVariant::佛 => &['佛', '仏'],
            OrthographicalVariant::澤 => &['澤', '沢'],
            OrthographicalVariant::塚 => &['塚', '塚'],
            OrthographicalVariant::恵 => &['恵', '惠'],
            OrthographicalVariant::穂 => &['穂', '穗'],
            OrthographicalVariant::梼 => &['梼', '檮'],
            OrthographicalVariant::蛍 => &['蛍', '螢'],
            OrthographicalVariant::與 => &['與', '与'],
            OrthographicalVariant::瀧 => &['瀧', '滝'],
            OrthographicalVariant::籠 => &['籠', '篭'],
            OrthographicalVariant::濱 => &['濱', '浜'],
            OrthographicalVariant::祗 => &['祗', '祇'],
            OrthographicalVariant::曾 => &['曾', '曽'],
        }
    }

    fn permutations(&self) -> Vec<(char, char)> {
        let characters = self.value();
        let mut permutations: Vec<(char, char)> = vec![];
        for n in 0..characters.len() {
            for m in 0..characters.len() {
                if n != m {
                    permutations.push((characters[n], characters[m]));
                }
            }
        }
        permutations
    }
}

pub struct OrthographicalVariantAdapter {
    pub variant_list: Vec<OrthographicalVariant>,
}

impl OrthographicalVariantAdapter {
    pub fn apply(self, input: &str, region_name: &str) -> Option<(String, String)> {
        // 必要なパターンのみを選別する
        let variant_list: Vec<&OrthographicalVariant> = self
            .variant_list
            .iter()
            .filter(|v| v.value().iter().any(|&c| input.contains(c)))
            .collect();
        if variant_list.is_empty() {
            return None;
        }

        // マッチ候補を容れておくためのVector
        let mut candidates: Vec<String> = vec![region_name.to_string()];
        // パターンを一つづつ検証していく
        for variant in variant_list {
            let mut semi_candidates: Vec<String> = vec![];
            // variantから順列を作成
            // ["ケ", "ヶ", "が"] -> (ケ, ヶ), (ケ, が), (ヶ, ケ), (ヶ, が), (が, ケ), (が, ヶ)
            for (a, b) in variant.permutations() {
                for candidate in candidates.iter().filter(|x| x.contains(a)) {
                    let modified_candidate = modify_specific_character(candidate, a, b);
                    if input.starts_with(&modified_candidate) {
                        // マッチすれば早期リターン
                        return Some((
                            region_name.to_string(),
                            input
                                .chars()
                                .skip(modified_candidate.chars().count())
                                .collect(),
                        ));
                    } else {
                        // マッチしなければsemi_candidatesに置き換え後の文字列をpush
                        semi_candidates.push(modified_candidate);
                    }
                }
            }
            candidates = semi_candidates;
            candidates.push(region_name.to_string());
        }
        None
    }
}

fn modify_specific_character(text: &str, from: char, to: char) -> String {
    text.chars()
        .map(|x| if x == from { to } else { x })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::parser::adapter::orthographical_variant_adapter::OrthographicalVariant;

    #[test]
    fn permutations() {
        let variant = OrthographicalVariant::ケ;
        assert_eq!(
            variant.permutations(),
            vec![
                ('ケ', 'ヶ'),
                ('ケ', 'が'),
                ('ケ', 'ガ'),
                ('ヶ', 'ケ'),
                ('ヶ', 'が'),
                ('ヶ', 'ガ'),
                ('が', 'ケ'),
                ('が', 'ヶ'),
                ('が', 'ガ'),
                ('ガ', 'ケ'),
                ('ガ', 'ヶ'),
                ('ガ', 'が'),
            ]
        );
    }
}
