use rand::seq::SliceRandom;
use rand::thread_rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameStatus {
    Playing,
    PlayerBust,
    DealerBust,
    PlayerWin,
    DealerWin,
    Push,
}

#[wasm_bindgen]
pub struct BlackjackGame {
    deck: Vec<u8>,
    player_hand: Vec<u8>,
    dealer_hand: Vec<u8>,
    status: GameStatus,
}

#[wasm_bindgen]
impl BlackjackGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> BlackjackGame {
        let mut game = BlackjackGame {
            deck: fresh_shuffled_deck(),
            player_hand: Vec::new(),
            dealer_hand: Vec::new(),
            status: GameStatus::Playing,
        };

        game.deal_initial_cards();
        game
    }

    pub fn reset(&mut self) {
        self.deck = fresh_shuffled_deck();
        self.player_hand.clear();
        self.dealer_hand.clear();
        self.status = GameStatus::Playing;
        self.deal_initial_cards();
    }

    pub fn hit(&mut self) -> Result<(), JsValue> {
        if self.status != GameStatus::Playing {
            return Err(JsValue::from_str("Game has ended. Please reset."));
        }

        let card = self.draw_card()?;
        self.player_hand.push(card);

        if hand_value(&self.player_hand) > 21 {
            self.status = GameStatus::PlayerBust;
        }

        Ok(())
    }

    pub fn stand(&mut self) -> Result<(), JsValue> {
        if self.status != GameStatus::Playing {
            return Err(JsValue::from_str("Game has ended. Please reset."));
        }

        while hand_value(&self.dealer_hand) < 17 {
            let card = self.draw_card()?;
            self.dealer_hand.push(card);
        }

        let player_total = hand_value(&self.player_hand);
        let dealer_total = hand_value(&self.dealer_hand);

        self.status = if dealer_total > 21 {
            GameStatus::DealerBust
        } else if player_total > dealer_total {
            GameStatus::PlayerWin
        } else if dealer_total > player_total {
            GameStatus::DealerWin
        } else {
            GameStatus::Push
        };

        Ok(())
    }

    pub fn status(&self) -> GameStatus {
        self.status
    }

    pub fn player_total(&self) -> u8 {
        hand_value(&self.player_hand)
    }

    pub fn dealer_total_visible(&self) -> u8 {
        if self.status == GameStatus::Playing {
            self.dealer_hand.first().copied().unwrap_or(0)
        } else {
            hand_value(&self.dealer_hand)
        }
    }

    pub fn player_cards(&self) -> Vec<u8> {
        self.player_hand.clone()
    }

    pub fn dealer_cards(&self) -> Vec<u8> {
        self.dealer_hand.clone()
    }

    fn deal_initial_cards(&mut self) {
        for _ in 0..2 {
            if let Ok(c) = self.draw_card() {
                self.player_hand.push(c);
            }
            if let Ok(c) = self.draw_card() {
                self.dealer_hand.push(c);
            }
        }
    }

    fn draw_card(&mut self) -> Result<u8, JsValue> {
        self.deck
            .pop()
            .ok_or_else(|| JsValue::from_str("Deck is empty"))
    }
}

fn fresh_shuffled_deck() -> Vec<u8> {
    let mut deck = Vec::with_capacity(52);
    for _suit in 0..4 {
        for rank in 1..=13 {
            deck.push(rank_to_value(rank));
        }
    }
    deck.shuffle(&mut thread_rng());
    deck
}

fn rank_to_value(rank: u8) -> u8 {
    match rank {
        1 => 11,
        11..=13 => 10,
        _ => rank,
    }
}

fn hand_value(cards: &[u8]) -> u8 {
    let mut total: u8 = cards.iter().sum();
    let mut aces = cards.iter().filter(|&&c| c == 11).count();

    while total > 21 && aces > 0 {
        total -= 10;
        aces -= 1;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ace_should_adjust_from_11_to_1() {
        let cards = vec![11, 9, 5];
        assert_eq!(hand_value(&cards), 15);
    }

    #[test]
    fn new_game_has_two_cards_each() {
        let game = BlackjackGame::new();
        assert_eq!(game.player_cards().len(), 2);
        assert_eq!(game.dealer_cards().len(), 2);
    }

    #[test]
    fn stand_ends_game() {
        let mut game = BlackjackGame::new();
        game.stand().unwrap();
        assert_ne!(game.status(), GameStatus::Playing);
    }
}
