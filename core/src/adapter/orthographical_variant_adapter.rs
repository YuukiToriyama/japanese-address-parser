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
    廣,
    婁,
    麴,
    炮,
    邇,
    遙,
    溪,
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
            廣 => &['廣', '広'],
            婁 => &['婁', '娄'],
            麴 => &['麴', '麹'],
            炮 => &['炮', '砲'],
            邇 => &['邇', '爾', '迩'],
            遙 => &['遙', '遥'],
            溪 => &['溪', '渓'],
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

struct TextCursor {
    body: Vec<char>,
    index: usize,
}

impl TextCursor {
    fn advance(&mut self) {
        self.index += 1;
    }

    fn current_char(&self) -> char {
        self.body[self.index]
    }

    fn is_terminated(&self) -> bool {
        self.index >= self.body.len()
    }
}

struct OrthographicalVariantMatcher {
    /// ユーザーから入力された文字列
    input: TextCursor,
    /// 比較対象となる文字列
    target: TextCursor,
    /// 表記揺れとして考慮する文字種のパターン群
    variants: Vec<OrthographicalVariant>,
}

impl OrthographicalVariantMatcher {
    pub fn new<T: Into<String>>(input: T, target: T, variants: Vec<OrthographicalVariant>) -> Self {
        let input = input.into();
        let target = target.into();
        Self {
            input: TextCursor {
                body: input.chars().collect(),
                index: 0,
            },
            target: TextCursor {
                body: target.chars().collect(),
                index: 0,
            },
            variants,
        }
    }

    pub fn matches(&mut self) -> bool {
        // targetを先頭から読み取っていく
        'outer_loop: while !self.target.is_terminated() {
            // targetには残りがあるのにinputを読み切ってしまった場合は、処理を終了する
            if self.input.is_terminated() {
                return false;
            }

            let input_char = self.input.current_char();
            let target_char = self.target.current_char();

            // 文字が一致する場合は、次の文字の評価に進む
            if input_char == target_char {
                self.input.advance();
                self.target.advance();
                continue;
            }

            // 文字が一致しない場合は、表記揺れの可能性を疑う
            for variant in &self.variants {
                // 同じ表記揺れパターンを共有している場合は、同じ文字であると評価して次の文字の評価に進む
                if variant.value().contains(&input_char) && variant.value().contains(&target_char) {
                    self.input.advance();
                    self.target.advance();
                    continue 'outer_loop;
                }
            }

            // 表記揺れを考慮しても文字が一致しない場合は、マッチ失敗としてfalseを返す
            return false;
        }

        // targetを最後まで読み切ったら、マッチ成功としてtrueを返す
        true
    }
}

#[cfg(test)]
mod matcher_tests {
    use crate::adapter::orthographical_variant_adapter::{
        OrthographicalVariant, OrthographicalVariantMatcher,
    };

    #[test]
    fn 比較対象より入力のほうが短い場合_falseを返す() {
        let mut matcher = OrthographicalVariantMatcher::new("千駄ケ谷", "千駄ケ谷四丁目", vec![]);
        assert_eq!(matcher.matches(), false);
    }

    #[test]
    fn 比較対象が入力に対して前方一致する場合_trueを返す() {
        let mut matcher =
            OrthographicalVariantMatcher::new("千駄ケ谷四丁目1-1", "千駄ケ谷四丁目", vec![]);
        assert_eq!(matcher.matches(), true);
    }

    #[test]
    fn 表記揺れを考慮したうえで比較対象が入力に対して前方一致する場合_trueを返す() {
        let mut matcher = OrthographicalVariantMatcher::new(
            "千駄ヶ谷四丁目1-1",
            "千駄ケ谷四丁目",
            vec![OrthographicalVariant::ケ],
        );
        assert_eq!(matcher.matches(), true);
    }

    #[test]
    fn 表記揺れを考慮しても文字が一致しない場合_falseを返す() {
        let mut matcher = OrthographicalVariantMatcher::new(
            "百駄ヶ谷四丁目1-1",
            "千駄ケ谷四丁目",
            vec![OrthographicalVariant::ケ],
        );
        assert_eq!(matcher.matches(), false);
    }

    #[test]
    fn 表記揺れがあるがvariantsに何も指定されていない場合_falseを返す() {
        let mut matcher =
            OrthographicalVariantMatcher::new("千駄ヶ谷四丁目", "千駄ケ谷四丁目", vec![]);
        assert_eq!(matcher.matches(), false);
    }

    #[test]
    fn 複数個の表記揺れを考慮したうえで比較対象が入力に対して前方一致する場合_trueを返す() {
        let mut matcher = OrthographicalVariantMatcher::new(
            "松が﨑御所之内町",
            "松ケ崎御所ノ内町",
            vec![
                OrthographicalVariant::崎,
                OrthographicalVariant::の,
                OrthographicalVariant::ケ,
            ],
        );
        assert_eq!(matcher.matches(), true);
    }
}

pub struct OrthographicalVariantAdapter {
    pub variant_list: Vec<OrthographicalVariant>,
}

impl OrthographicalVariantAdapter {
    pub fn apply(&self, input: &str, region_name: &str) -> Option<(String, String)> {
        // 必要最低限のパターンのみを選別する
        let variants = self.filter_variants(region_name);
        if variants.is_empty() {
            return None;
        }
        self.match_with_variants(input, region_name, variants)
    }

    /// マッチ候補の文字列(target)と表記揺れパターン(self.variant_list)を照らし合わせ、マッチ候補の文字列に含まれる文字のパターンのみを選別する
    fn filter_variants(&self, target: &str) -> Vec<&OrthographicalVariant> {
        // マッチ候補の文字列
        self.variant_list
            .iter()
            .filter(|v| v.value().iter().any(|&c| target.contains(c)))
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
