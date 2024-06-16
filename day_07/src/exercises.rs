use std::cmp::Ordering;
use std::collections::HashMap;

const CARDS: [&str; 13] = ["A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2"];
const CARDS2: [&str; 13] = ["A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2", "J"];


#[derive(Debug, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
    Nothing,
}

pub fn part1(input: &str) -> u32 {

    let mut hands = input.split('\n').into_iter()
        .map(|l| {
            let v = l.trim().split(" ").collect::<Vec<&str>>();
            (v[0].trim(),v[1].trim())
        }).collect::<Vec<(&str, &str)>>();




    hands.sort_by(|a,b| compare_hands(a, b));
    // let hands1: Vec<(&str, HandType)> = hands.into_iter().map(|hand| (hand, check_hand_type(hand))).collect();


    let len = hands.len();
    let mut res = 0u32;
    for i in (1..=len).rev() {
        // println!("Item #{} K={} V={} res={}", len-i, hands[len-i].0, hands[len-i].1, res);
        res += hands[len-i].1.parse::<u32>().expect("*** Not a number!!!!") * i as u32;
    }
    res

}

pub fn part2(input: &str) -> u32 {
    let mut hands = input.split('\n').into_iter()
        .map(|l| {
            let v = l.trim().split(" ").collect::<Vec<&str>>();
            (v[0].trim(),v[1].trim())
        }).collect::<Vec<(&str, &str)>>();
    
    hands.sort_by(|a,b| compare_hands2(a, b));

    let len = hands.len();
    let mut res = 0u32;
    for i in (1..=len).rev() {
        println!("Item #{} K={} V={} res={}", len-i, hands[len-i].0, hands[len-i].1, res);
        res += hands[len-i].1.parse::<u32>().expect("*** Not a number!!!!") * i as u32;
    }
    res
}


fn second_ordering(hand1: &str, hand2: & str) -> i32 {
    if compare_cards(&hand1[0..1], &hand2[0..1]) > 0 {
        1
    } else if compare_cards(&hand1[0..1], &hand2[0..1]) < 0 {
        -1
    }else if compare_cards(&hand1[1..2], &hand2[1..2]) > 0 {
        1
    } else if compare_cards(&hand1[1..2], &hand2[1..2]) < 0 {
        -1
    }else if compare_cards(&hand1[2..3], &hand2[2..3]) > 0 {
        1
    } else if compare_cards(&hand1[2..3], &hand2[2..3]) < 0 {
        -1
    } else if compare_cards(&hand1[3..4], &hand2[3..4]) > 0 {
        1
    } else if compare_cards(&hand1[3..4], &hand2[3..4]) < 0 {
        -1
    } else if compare_cards(&hand1[4..5], &hand2[4..5]) > 0 {
        1
    } else if compare_cards(&hand1[4..5], &hand2[4..5]) < 0 {
        -1
    } else {
        0
    }

}



fn second_ordering2(hand1: &str, hand2: & str) -> i32 {
    if compare_cards2(&hand1[0..1], &hand2[0..1]) > 0 {
        1
    } else if compare_cards2(&hand1[0..1], &hand2[0..1]) < 0 {
        -1
    }else if compare_cards2(&hand1[1..2], &hand2[1..2]) > 0 {
        1
    } else if compare_cards2(&hand1[1..2], &hand2[1..2]) < 0 {
        -1
    }else if compare_cards2(&hand1[2..3], &hand2[2..3]) > 0 {
        1
    } else if compare_cards2(&hand1[2..3], &hand2[2..3]) < 0 {
        -1
    } else if compare_cards2(&hand1[3..4], &hand2[3..4]) > 0 {
        1
    } else if compare_cards2(&hand1[3..4], &hand2[3..4]) < 0 {
        -1
    } else if compare_cards2(&hand1[4..5], &hand2[4..5]) > 0 {
        1
    } else if compare_cards2(&hand1[4..5], &hand2[4..5]) < 0 {
        -1
    } else {
        0
    }

}


fn compare_hands(hand1: &(&str,  &str), hand2: &(&str,  &str)) -> Ordering {
    let hand_type1 = check_hand_type(hand1.0);
    let hand_type2 = check_hand_type(hand2.0);
    let res = hand_type1 as i8 - hand_type2 as i8;
    if res == 0 {
        //Ordering::Equal
        if second_ordering(hand1.0, hand2.0) > 0 {
            Ordering::Greater
        } else if second_ordering(hand1.0, hand2.0) < 0 {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    } else if res < 0 {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}


fn compare_hands2(hand1: &(&str,  &str), hand2: &(&str,  &str)) -> Ordering {
    let hand_type1 = check_hand_type2(hand1.0);
    let hand_type2 = check_hand_type2(hand2.0);
    let res = hand_type1 as i8 - hand_type2 as i8;
    if res == 0 {
        //Ordering::Equal
        if second_ordering2(hand1.0, hand2.0) > 0 {
            Ordering::Greater
        } else if second_ordering2(hand1.0, hand2.0) < 0 {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    } else if res < 0 {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}




fn compare_cards(a: &str, b: &str) -> i8 {
    let pos_a = find_card(a);
    let pos_b = find_card(b);
    match (pos_a, pos_b) {
        (Ok(index_a), Ok(index_b)) => index_a as i8 - index_b as i8, // Return the difference in indices
        (_, _) => panic!("One or both of the cards were not found in CARDS"), // Handle error case
    }
}

fn compare_cards2(a: &str, b: &str) -> i8 {
    let pos_a = find_card2(a);
    let pos_b = find_card2(b);
    match (pos_a, pos_b) {
        (Ok(index_a), Ok(index_b)) => index_a as i8 - index_b as i8, // Return the difference in indices
        (_, _) => panic!("One or both of the cards were not found in CARDS2"), // Handle error case
    }
}


fn find_card(card: &str) -> Result<usize, &'static str> {
    for i in 0..CARDS.len() {
        if CARDS[i] == card {
            return Ok(i);
        }
    }
    return Err("find_card: Could not find card.");
}

fn find_card2(card: &str) -> Result<usize, &'static str> {
    for i in 0..CARDS2.len() {
        if CARDS2[i] == card {
            return Ok(i);
        }
    }
    return Err("find_card: Could not find card.");
}
fn check_hand_type(hand: &str) -> HandType {
    let mut cards = HashMap::new();
    for i in 0..5 {
        cards.insert(hand.as_bytes()[i] as char, count_cards(hand, hand.as_bytes()[i] as char));
    }
    match cards.len() {
        1 => HandType::FiveOfAKind,
        2 => {
            let values = cards.values();
            if values.filter(|p|  **p == 1 || **p == 4).count() > 0 {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        }
        3 => {
            let three = cards.values().filter(|l| **l == 3).count() == 1;
            if three {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        }
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => HandType::Nothing
    }

}


fn check_hand_type2(hand: &str) -> HandType {
    let mut stacks = HashMap::new();
    for i in 0..5 {
        stacks.insert(hand.as_bytes()[i] as char, count_cards(hand, hand.as_bytes()[i] as char));
    }
    let js = stacks.get(&'J').or(Some(&0)).unwrap();
    match stacks.len() {
        1 => HandType::FiveOfAKind,
        2 => {
            let values = stacks.values();
            if *js > 0 {
                HandType::FiveOfAKind
            } else if values.filter(|p|  **p == 1 || **p == 4).count() > 0 {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        },
        3 => {
            let three = stacks.values().filter(|l| **l == 3).count() == 1;
            
            if *js == 2 || (three && *js == 1) {
                HandType::FourOfAKind
            } else if *js == 3 {
                HandType::FourOfAKind
            } else if three && *js == 0{
                HandType::ThreeOfAKind
            } else if *js == 1 {
                HandType::FullHouse
            } else {
                HandType::TwoPair
            }
        },
        4 => {
            if *js > 0 {
                HandType::ThreeOfAKind
            } else {
                HandType::OnePair
            }
        },
        5 => {
            if *js == 1 {
                HandType::OnePair
            } else {
                HandType::HighCard
            }
        },
        _ => HandType::Nothing
    }

}


fn count_cards(hand: &str, card: char) -> usize {
    hand.rmatches(card).collect::<Vec<&str>>().len()
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use crate::exercises::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    const INPUT2: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn scn_compare_01() {
        let a = "A";
        let b = "2";
        let result = compare_cards(a, b);
        assert!(result < 0);
    }

    #[test]
    fn scn_compare_02() {
        let a = "Q";
        let b = "9";
        let result = compare_cards(a, b);
        assert!(result < 0);
    }

    #[test]
    fn scn_compare_02_01() {
        let a = "9";
        let b = "Q";
        let result = compare_cards(a, b);
        assert!(result > 0);
    }


    #[test]
    fn scn_second_ordering_is_first_greater_01() {
        let a = "AA";
        let b = "22";
        let result = second_ordering(a, b);
        assert_eq!(result, -1);
    }
    #[test]
    fn scn_second_ordering_is_first_greater_02() {
        let a = "2222A";
        let b = "2222K";
        let result = second_ordering(a, b);
        assert_eq!(result, -1);
    }

    #[test]
    fn scn_second_ordering_is_first_greater_03() {
        let a = "22222";
        let b = "22223";
        let result = second_ordering(a, b);
        assert_eq!(result, 1);
    }

    #[test]
    fn scn_second_ordering_is_first_greater_04() {
        let a = "T55J5";
        let b = "QQQJA";
        let result = second_ordering(a, b);
        assert_eq!(result, 1);
    }

    // compare_cards(&hand1[0..1], &hand2[0..1]
    #[test]
    fn scn_second_ordering_is_first_greater_05() {
        let a = "T55J5";
        let b = "QQQJA";
        let result = compare_cards(&a[0..1], &b[0..1]);
        assert!(result > 0);
    }

    #[test]
    fn scn_second_ordering_is_first_greater_06() {
        let a = "T55J5";
        let b = "QQQJA";
        let result = compare_cards(&a[0..1], &b[0..1]);
        assert!(result > 0);
    }

    #[test]
    fn scn_second_ordering_01() {
        let a = "2AAAA";
        let b = "AAAA2";
        let result = second_ordering(a, b);
        assert_eq!(result, 1);
    }

    #[test]
    fn scn_second_ordering_02() {
        let a = "QQQQ8";
        let b = "9Q999";
        let result = second_ordering(a, b);
        assert_eq!(result, -1);
    }

    #[test]
    #[should_panic]
    fn scn_compare_03() {
        let a = "A";
        let b = "0";
        let _result = compare_cards(a, b);
    }

    #[test]
    fn scn_check_hand_type_01() {
        let hand = "AAAAA";
        let res = check_hand_type(hand);
        assert_eq!(HandType::FiveOfAKind, res);
    }

    #[test]
    fn scn_check_hand_type_02() {
        let hand = "AAAQQ";
        let res = check_hand_type(hand);
        assert_eq!(HandType::FullHouse, res);
    }

    #[test]
    fn scn_check_hand_type_03() {
        let hand = "AAQAQ";
        let res = check_hand_type(hand);
        assert_eq!(HandType::FullHouse, res);
    }

    #[test]
    fn scn_check_hand_type_04() {
        let hand = "QQQAA";
        let res = check_hand_type(hand);
        assert_eq!(HandType::FullHouse, res);
    }

    #[test]
    fn scn_check_hand_type_05() {
        let hand = "QQQJA";
        let res = check_hand_type(hand);
        assert_eq!(HandType::ThreeOfAKind, res);
    }

    #[test]
    fn scn_check_hand_type_06() {
        let hand = "QJQQA";
        let res = check_hand_type(hand);
        assert_eq!(HandType::ThreeOfAKind, res);
    }

    #[test]
    fn scn_check_hand_type_07() {
        let hand = "JAQQQ";
        let res = check_hand_type(hand);
        assert_eq!(HandType::ThreeOfAKind, res);
    }
    #[test]
    fn scn_check_hand_type_08() {
        let hand = "JAQAQ";
        let res = check_hand_type(hand);
        assert_eq!(HandType::TwoPair, res);
    }
    #[test]
    fn scn_check_hand_type_09() {
        let hand = "JAQAK";
        let res = check_hand_type(hand);
        assert_eq!(HandType::OnePair, res);
    }
    #[test]
    fn scn_check_hand_type_10() {
        let hand = "23456";
        let res = check_hand_type(hand);
        assert_eq!(HandType::HighCard, res);
    }

    #[test]
    fn scn_compare_hands_01() {
        let hand1 = ("AAAAA", "2");
        let hand2 = ("AAQQQ", "3");
        let res = compare_hands(&hand1, &hand2);
        assert_eq!(res, Ordering::Less);
    }

    #[test]
    fn scn_compare_hands_02() {
        let hand1 = ("AAQQQ", "2");
        let hand2 = ("AAAAA", "2");
        let res = compare_hands(&hand1, &hand2);
        assert_eq!(res, Ordering::Greater);
    }


    #[test]
    fn scn_compare_hands_03() {
        let hand1 = ("22222", "2");
        let hand2 = ("22222", "2");
        let res = compare_hands(&hand1, &hand2);
        assert_eq!(res, Ordering::Equal);
    }


    #[test]
    fn scn_compare_hands_04() {
        let hand1 = ("QQQJJ", "2");
        let hand2 = ("QQQQ8", "2");
        let res = compare_hands(&hand1, &hand2);
        assert_eq!(res, Ordering::Greater);
    }

    #[test]
    fn scn_compare_hands_05() {
        let hand1 = ("QQQQ8", "2");
        let hand2 = ("QQQJJ", "2");
        let res = compare_hands(&hand1, &hand2);
        assert_eq!(res, Ordering::Less);
    }
    #[test]
    fn scn_compare_hands_06() {
        let hand1 = ("9Q999", "2");
        let hand2 = ("QQQQ8", "2");
        let res = compare_hands(&hand1, &hand2);
        assert_eq!(res, Ordering::Greater);
    }
    #[test]
    fn scn_compare_hands_07() {
        let hand1 = ("QQQQ8", "2");
        let hand2 = ("9Q999", "2");
        let res = compare_hands(&hand1, &hand2);
        assert_eq!(res, Ordering::Less);
    }


    #[test]
    fn scn_p1_01() {
        let result = part1(INPUT);
        assert_eq!(result, 6440);
    }

    #[test]
    fn scn_check_hand_type2_01() {
        let hand = "AJAJQ";
        let res = check_hand_type2(&hand);
        assert_eq!(res, HandType::FourOfAKind);
    }

    #[test]
    fn scn_check_hand_type2_02() {
        let hand = "AJAJA";
        let res = check_hand_type2(&hand);
        assert_eq!(res, HandType::FiveOfAKind);
    }
    #[test]
    fn scn_check_hand_type2_03() {
        let hand = "AJAJJ";
        let res = check_hand_type2(&hand);
        assert_eq!(res, HandType::FiveOfAKind);
    }
    #[test]
    fn scn_check_hand_type2_04() {
        let hand = "AJAQQ";
        let res = check_hand_type2(&hand);
        assert_eq!(res, HandType::FullHouse);
    }
    #[test]
    fn scn_check_hand_type2_05() {
        let hand = "AQJK5";
        let res = check_hand_type2(&hand);
        assert_eq!(res, HandType::OnePair);
    }

    #[test]
    fn scn_part2_01() {
        let res = part2(INPUT);
        assert_eq!(res, 5905);
    }
}