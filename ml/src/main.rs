use std::vec;

use tokenizers::{tokenizer::{Result, Tokenizer}, PostProcessor, TruncationParams};
use tch::Tensor;

fn main() {
    let t = Tensor::of_slice(&[3, 1, 4, 1, 5]);
    let t = t * 2;
    t.print();

    let sentence = "Google is an American multinational technology company focusing on search engine technology, online advertising, cloud computing, computer software, quantum computing, e-commerce, artificial intelligence, and consumer electronics. It has been referred to as \"the most powerful company in the world\" and one of the world's most valuable brands due to its market dominance, data collection, and technological advantages in the area of artificial intelligence. Its parent company Alphabet is considered one of the Big Five American information technology companies, alongside Amazon, Apple, Meta, and Microsoft.".to_string();
    // let sentence = "Традиционная задача любой поисковой системы — находить нужную информацию в интернете. В современном мире эта задача расширилась. Интернет уже нельзя отделить от реальности вокруг нас, и поисковые системы сейчас ищут по всему окружающему миру. И не только ищут, но и умеют подсказывать во всех жизненных ситуациях. Современный поиск понимает желания каждого пользователя и знает, что его окружает. Поэтому он может подсказать каждому человеку то, что подойдет именно ему. Что почитать, куда пойти поесть, какую музыку послушать, как быстрее попасть домой, где купить самые дешевые билеты. Поиск всегда был и остается главным сервисом Яндекса, однако сегодня мы понимаем его роль более широко — это универсальный помощник, навигатор по всему, что нас окружает. Поиск теперь — и подсказка в нужный момент, и помощь в выборе, и возможность совершить действие.".to_string();
    let mut tokenizer = Tokenizer::from_pretrained("JulesBelveze/t5-small-headline-generator", None).unwrap();
    // let mut trunc =
    let encoding = tokenizer
        .encode(sentence, false)
        .unwrap();

    let model = tokenizer.get_model();
    // let pp = tokenizer.get_post_processor().unwrap();
    // dbg!(pp);
    // let yy = pp.process_encodings(vec![encoding.clone()], false).unwrap();
    // dbg!(yy);
    // let pp = tokenizer.post_process(encoding, pair_encoding, add_special_tokens);
    let ids = encoding.get_ids();

    println!("{:?}", ids);
    // let qq = tokenizer.decode(ids.to_owned(), true).unwrap();
    // dbg!(qq);

    // let tokenizer = Tokenizer::from_pretrained("IlyaGusev/rugpt3medium_sum_gazeta", None).unwrap();

    // let t = "Традиционная задача любой поисковой системы — находить нужную информацию в интернете. В современном мире эта задача расширилась. Интернет уже нельзя отделить от реальности вокруг нас, и поисковые системы сейчас ищут по всему окружающему миру. И не только ищут, но и умеют подсказывать во всех жизненных ситуациях. Современный поиск понимает желания каждого пользователя и знает, что его окружает. Поэтому он может подсказать каждому человеку то, что подойдет именно ему. Что почитать, куда пойти поесть, какую музыку послушать, как быстрее попасть домой, где купить самые дешевые билеты.".to_string();
    // let model = tokenizer.get_model();
    // // model.
    // dbg!(model);
    // let encoding = tokenizer.encode(t, false).unwrap();
    // let ids = encoding.get_ids();
    // let qq = tokenizer.decode(ids.to_owned(), true).unwrap();
    // println!("{:?}", encoding.get_tokens());
    // println!("{:?}", qq);
}
