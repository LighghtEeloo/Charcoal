use super::*;

pub struct Sentence;

impl Select for Sentence {
    type Target = Vec<(String, String)>;

    fn select(elem: ElementRef, word_query: &WordQuery) -> anyhow::Result<Self::Target> {
        let sel = Selector::parse("#bilingual.trans-container li").unwrap();
        Ok(elem
            .select(&sel)
            .filter_map(|child| Sen::select(child, word_query).ok())
            .collect())
    }
}

struct Sen;
const PUNCTUATORS: &[char; 10] = &['.', ',', '\"', '\'', '?', '!', ':', '-', '<', '>'];

impl Select for Sen {
    type Target = (String, String);

    fn select(elem: ElementRef, word_query: &WordQuery) -> anyhow::Result<Self::Target> {
        let sel = Selector::parse("p").unwrap();
        let mut iter = elem.select(&sel);

        let mut extract_to_vec = |msg| -> Vec<String> {
            iter.next()
                .expect(msg)
                .text()
                .filter_map(trim_str)
                .collect()
        };

        fn western_concat(vec: Vec<String>) -> String {
            let mut ori = String::new();
            let mut ori_iter = vec.into_iter();
            if let Some(s) = ori_iter.next() {
                ori.push_str(&s)
            }
            for mut s in ori_iter {
                if !s.starts_with(PUNCTUATORS) {
                    s.insert(0, ' ')
                }
                ori.push_str(&s)
            }
            ori
        }

        fn eastern_concat(vec: Vec<String>) -> String {
            vec.join("")
        }

        let ori_vec = extract_to_vec("No ori found in sentence");
        let trans_vec = extract_to_vec("No trans found in sentence");

        let (ori, trans) = if word_query.is_western() {
            (western_concat(ori_vec), eastern_concat(trans_vec))
        } else {
            (eastern_concat(ori_vec), western_concat(trans_vec))
        };
        Ok((ori, trans))
    }
}
