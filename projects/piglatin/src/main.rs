fn main() {

    let word = "first apple";

    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let mut pig_words = Vec::new();

    for i in word.split_whitespace() {
        let first_char = i.chars().next().unwrap();
        if vowels.contains(&first_char) {
            let vowel_word = format!("{}-{}", i, "hay");
            pig_words.push(vowel_word);
        } else {
            let trunc_word = i[1..].to_string();
            let const_word = format!("{}-{}{}", trunc_word, first_char, "ay");
            pig_words.push(const_word);
        }
    }

    println!("{:?}", pig_words.join(" "));
}


