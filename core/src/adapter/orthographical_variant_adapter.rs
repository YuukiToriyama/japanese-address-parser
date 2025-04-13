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
    鰺,
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
    國,
    鉋,
    鷆,
    斑,
    櫻,
    櫟,
    冨,
    諫,
    驒,
    鶯,
    條,
    婁,
}

impl OrthographicalVariant {
    fn value(&self) -> &[char] {
        use OrthographicalVariant::*;
        match self {
            の => &['の', 'ノ', '之'],
            ツ => &['ツ', 'ッ'],
            ケ => &['ケ', 'ヶ', 'が', 'ガ'],
            薮 => &['薮', '藪', '籔'],
            崎 => &['崎', '﨑'],
            檜 => &['桧', '檜'],
            龍 => &['龍', '竜'],
            竈 => &['竈', '竃', '釜', '釡'],
            嶋 => &['嶋', '島'],
            舘 => &['舘', '館'],
            鰺 => &['鰺', '鯵'],
            脊 => &['脊', '背'],
            渕 => &['渕', '淵'],
            己 => &['己', '巳'],
            槇 => &['槇', '槙'],
            治 => &['治', '冶'],
            佛 => &['佛', '仏'],
            澤 => &['澤', '沢'],
            塚 => &['塚', '塚'],
            恵 => &['恵', '惠'],
            穂 => &['穂', '穗'],
            梼 => &['梼', '檮'],
            蛍 => &['蛍', '螢'],
            與 => &['與', '与'],
            瀧 => &['瀧', '滝'],
            籠 => &['籠', '篭'],
            濱 => &['濱', '浜'],
            祗 => &['祗', '祇'],
            曾 => &['曾', '曽'],
            國 => &['國', '国'],
            鉋 => &['鉋', '飽'],
            鷆 => &['鷆', '鷏'],
            斑 => &['斑', '班'],
            櫻 => &['櫻', '桜'],
            櫟 => &['櫟', '擽'],
            冨 => &['冨', '富'],
            諫 => &['諫', '諌'],
            驒 => &['驒', '騨'],
            鶯 => &['鶯', '鴬'],
            條 => &['條', '条'],
            婁 => &['婁', '娄'],
        }
    }

    fn permutations(&self) -> Vec<(char, char)> {
        let characters = self.value();
        let mut permutations = Vec::with_capacity(characters.len() * (characters.len() - 1));
        for &a in characters {
            for &b in characters {
                if a != b {
                    permutations.push((a, b));
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
        let variants = self.filter_variants(input);
        if variants.is_empty() {
            return None;
        }
        self.match_with_variants(input, region_name, variants)
    }

    fn filter_variants(&self, input: &str) -> Vec<&OrthographicalVariant> {
        // 必要なパターンのみを選別する
        self.variant_list
            .iter()
            .filter(|v| v.value().iter().any(|&c| input.contains(c)))
            .collect()
    }

    fn match_with_variants(
        &self,
        input: &str,
        target: &str,
        variants: Vec<&OrthographicalVariant>,
    ) -> Option<(String, String)> {
        // マッチ候補を容れておくためのVector
        let mut candidates = vec![target.to_string()];
        // パターンを一つづつ検証していく
        for variant in variants {
            let mut semi_candidates = vec![];
            // variantから順列を作成
            // ["ケ", "ヶ", "が"] -> (ケ, ヶ), (ケ, が), (ヶ, ケ), (ヶ, が), (が, ケ), (が, ヶ)
            for (a, b) in variant.permutations() {
                for candidate in candidates.iter().filter(|x| x.contains(a)) {
                    let modified_candidate = modify_specific_character(candidate, a, b);
                    if input.starts_with(&modified_candidate) {
                        // マッチすれば早期リターン
                        return Some((
                            target.to_string(),
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
            candidates.push(target.to_string());
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
    use crate::adapter::orthographical_variant_adapter::OrthographicalVariant;

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
