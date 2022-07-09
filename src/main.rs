#![windows_subsystem = "windows"] // hide console window on Windows

use rand::Rng;
use eframe::egui;

struct Variables {
    run: bool,
    done:bool, 

    win_probability: f64,
    risk: f64,
    reward: f64,

    balance: f64,
    batch_size: f64,
    batch_target_mult: f64,
    final_target: f64,
    
    bet_amount: f64,
    bet_multiplier: f64,

    iterations: usize,

    success_rate_martingale:f64,
    martingale_average_bet_number_until_win: f64,
    martingale_average_bet_number_until_loss: f64,

    success_rate_inverse_martingale:f64,
    inverse_martingale_average_bet_number_until_win: f64,
    inverse_martingale_average_bet_number_until_loss: f64,

    can_exit: bool,
    is_exiting: bool,
}
impl Variables {
    fn default() -> Self {
        return Self {
            run: false,
            done: false, 

            win_probability: 50.0,
            risk: 1.0,
            reward: 1.0,

            balance: 1000.0,
            batch_size: 1000.0,
            batch_target_mult: 2.0,
            final_target: 2000.0,

            bet_amount: 100.0,
            bet_multiplier: 2.0,

            iterations: 1000,

            success_rate_martingale: 0.0,
            martingale_average_bet_number_until_win: 0.0,
            martingale_average_bet_number_until_loss: 0.0,

            success_rate_inverse_martingale: 0.0,
            inverse_martingale_average_bet_number_until_win: 0.0,
            inverse_martingale_average_bet_number_until_loss: 0.0,

            can_exit: false,
            is_exiting: false,
        }
    }
}
impl eframe::App for Variables {

    //Ran on exit
    fn on_exit_event(&mut self) -> bool {
        self.is_exiting = true;
        return self.can_exit;
    }
    
    //Ran every frame
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame){
    
        //Run main window
        egui::CentralPanel::default().show(ctx, |ui| {

            //Change settings or run simulation
            if self.run == false {
                
                    //Change Settings 
                    ui.horizontal(|ui| {
                        ui.label("Win probability(%):");
                        ui.add(egui::DragValue::new(&mut self.win_probability).speed(1));
                        if self.win_probability < 1.0 {self.win_probability = 1.0;}
                        if self.win_probability > 99.0 {self.win_probability = 99.0;}

                        ui.label("Risk:");
                        ui.add(egui::DragValue::new(&mut self.risk).speed(0.01));
                        if self.risk < 1.0 {self.risk = 1.0;}

                        ui.label("Reward:");
                        ui.add(egui::DragValue::new(&mut self.reward).speed(0.01));
                        if self.reward < 1.0 {self.reward = 1.0;}
                    });
                    ui.horizontal(|ui| {

                        ui.label("Starting balance($):");
                        ui.add(egui::DragValue::new(&mut self.balance).speed(10));
                        if self.balance < 1.0 {self.balance = 1.0;}

                        ui.label("Batch Size($):");
                        ui.add(egui::DragValue::new(&mut self.batch_size).speed(10));
                        if self.batch_size < 1.0 {self.batch_size = 1.0;}
                        
                        ui.label("Batch Multiplier target:");
                        ui.add(egui::DragValue::new(&mut self.batch_target_mult).speed(0.01));
                        if self.batch_target_mult < 1.01 {self.batch_target_mult = 1.01;}

                        ui.label("Target($):");
                        ui.add(egui::DragValue::new(&mut self.final_target).speed(10));
                        if self.final_target < 1.0 {self.final_target = 1.0;}
                        
                        ui.label("Starting Bet Amount($):");
                        ui.add(egui::DragValue::new(&mut self.bet_amount).speed(1));
                        if self.bet_amount < 1.0 {self.bet_amount = 1.0;}
                        if self.batch_size < 1.0 {self.bet_amount = self.batch_size;}

                    });

                    ui.horizontal(|ui| {
                        ui.label("Bet multiplier :");
                        ui.add(egui::DragValue::new(&mut self.bet_multiplier).speed(0.1));
                        if self.bet_multiplier < 1.0 {self.bet_multiplier = 1.0;}
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Iterations:");
                        ui.add(egui::DragValue::new(&mut self.iterations).speed(1));
                        if self.iterations < 1 {self.iterations = 1;}

                        if ui.button("Run").clicked() {
                            self.run = true;
                        }
                    });
                    
            }
            else{
                //Simulation done
                if self.done == false {

                    let balance_start = self.balance;
                    let batch_size_start = self.batch_size;
                    let bet_start = self.bet_amount;


                    //Martingale
                    let mut outcome_win:u64 = 0;
                    let mut outcome_win_bet_numbers:u64 = 0;
                    let mut outcome_loss:u64 = 0;
                    let mut outcome_loss_bet_numbers:u64 = 0;

                    for _ in 0..=self.iterations{

                        //Number of bets 
                        let mut bet_number:u64 = 0;

                        //While balance is still avaliable and hasnt reached the target
                        while self.balance > 0.0 && self.balance < self.final_target {

                            //Get batch from the total balance
                            self.batch_size = if batch_size_start > self.balance {self.balance} else {batch_size_start};
                            self.balance -= self.batch_size;

                            //Limit bet amount to the batch size
                            self.bet_amount = if bet_start > self.batch_size {self.batch_size} else {bet_start};
                            
                            //For each batch until all gets lost or batch target multiplier is reached
                            while self.batch_size > 0.0 && self.batch_size < (batch_size_start * self.batch_target_mult) {
                
                                //Random value
                                let roll:f64 = rand::thread_rng().gen_range(0.0..=100.0);

                                //Win
                                if roll <= self.win_probability{

                                    self.batch_size += self.bet_amount * self.reward;
                                    self.bet_amount = bet_start;
                                }

                                //Loss
                                else{
                                    self.batch_size -= self.bet_amount * self.risk;
                                    self.bet_amount *= self.bet_multiplier;    
                                } 

                                //Limit bet amount to needed to reach target
                                if self.bet_amount > ((batch_size_start * self.batch_target_mult) - self.batch_size) {
                                    self.bet_amount = (batch_size_start * self.batch_target_mult) - self.batch_size;
                                }

                                //Limit bet amount to batch balance
                                if self.bet_amount > self.batch_size {
                                    self.bet_amount = self.batch_size;
                                }
                                
                                bet_number += 1;
                            }
                            //Add the batch back into to the total balance
                            self.balance += self.batch_size;
                            self.batch_size = batch_size_start;
                        }

                        if self.balance >= self.final_target{
                            outcome_win += 1;
                            outcome_win_bet_numbers += bet_number;
                         }
                        else
                        {
                            outcome_loss += 1;
                            outcome_loss_bet_numbers += bet_number;
                        }
                        
                        //Reset balance and bet amount each iteration
                        self.balance = balance_start;
                        self.bet_amount = bet_start;
                    }   
                
                    self.success_rate_martingale = (outcome_win as f64)/(outcome_win as f64 + outcome_loss as f64) * 100.0;
                    self.martingale_average_bet_number_until_win = (outcome_win_bet_numbers as f64)/(outcome_win as f64);
                    self.martingale_average_bet_number_until_loss = (outcome_loss_bet_numbers as f64)/(outcome_loss as f64);


                    //Inverse Martingale
                    outcome_win = 0;
                    outcome_win_bet_numbers = 0;
                    outcome_loss = 0;
                    outcome_loss_bet_numbers = 0;

                    for _ in 0..=self.iterations{

                        //Number of bets 
                        let mut bet_number:u64 = 0;

                        while self.balance > 0.0 && self.balance < self.final_target {

                            //Get batch from the total balance
                            self.batch_size = if batch_size_start > self.balance {self.balance} else {batch_size_start};
                            self.balance -= self.batch_size;

                            //Limit bet amount to the batch size
                            self.bet_amount = if bet_start > self.batch_size {self.batch_size} else {bet_start};

                            //For each batch until all gets lost or batch target multiplier is reached
                            while self.batch_size > 0.0 && self.batch_size < (batch_size_start * self.batch_target_mult) {
                
                                //Random value
                                let roll:f64 = rand::thread_rng().gen_range(0.0..=100.0);

                                //Win
                                if roll <= self.win_probability{

                                    self.batch_size += self.bet_amount * self.reward;
                                    self.bet_amount *= self.bet_multiplier;
                                }

                                //Loss
                                else{
                                    self.batch_size -= self.bet_amount * self.risk;
                                    self.bet_amount = bet_start;   
                                } 

                                //Limit bet amount needed to reach batch target
                                if self.bet_amount > ((batch_size_start * self.batch_target_mult) - self.batch_size) {
                                    self.bet_amount = (batch_size_start * self.batch_target_mult) - self.batch_size;
                                }

                                //Limit bet amount to batch balance
                                if self.bet_amount > self.batch_size {
                                    self.bet_amount = self.batch_size;
                                }
                                
                                bet_number += 1;
                            }

                            //Add the batch to the balance and reset it
                            self.balance += self.batch_size;
                            self.batch_size = batch_size_start;

                        }
                        if self.balance >= self.final_target{
                            outcome_win += 1;
                            outcome_win_bet_numbers += bet_number;
                         }
                        else
                        {
                            outcome_loss += 1;
                            outcome_loss_bet_numbers += bet_number;
                        }
                        
                        //Reset balance and bet amount each iteration
                        self.balance = balance_start;
                        self.bet_amount = bet_start;
                    }   
                
                    self.success_rate_inverse_martingale = (outcome_win as f64)/(outcome_win as f64 + outcome_loss as f64) * 100.0;
                    self.inverse_martingale_average_bet_number_until_win = (outcome_win_bet_numbers as f64)/(outcome_win as f64);
                    self.inverse_martingale_average_bet_number_until_loss = (outcome_loss_bet_numbers as f64)/(outcome_loss as f64);


                    self.done = true;
                }
                //Display Results
                else{
                    ui.label("Martingale:");
                    ui.label("Success Rate: ".to_owned() + &self.success_rate_martingale.to_string() + "%");
                    ui.label("Average bets until target reached ".to_owned() + &self.martingale_average_bet_number_until_win.to_string());
                    ui.label("Average bets until lose all ".to_owned() + &self.martingale_average_bet_number_until_loss.to_string());

                    ui.label("\n\nInverse Martingale:");
                    ui.label("Success Rate: ".to_owned() + &self.success_rate_inverse_martingale.to_string() + "%");
                    ui.label("Average bets until target reached ".to_owned() + &self.inverse_martingale_average_bet_number_until_win.to_string());
                    ui.label("Average bets until lose all ".to_owned() + &self.inverse_martingale_average_bet_number_until_loss.to_string());

                    if ui.button("Reset").clicked(){

                        self.success_rate_martingale = 0.0;
                        self.martingale_average_bet_number_until_win = 0.0;
                        self.martingale_average_bet_number_until_loss = 0.0;

                        self.success_rate_inverse_martingale = 0.0;
                        self.inverse_martingale_average_bet_number_until_win = 0.0;
                        self.inverse_martingale_average_bet_number_until_loss = 0.0;

                        self.done = false;
                        self.run = false;
                    }
                }
            }
        });

        //Exit window
        if self.is_exiting {
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Yes").clicked() {
                            self.can_exit = true;
                            frame.quit();
                        }
                        if ui.button("No").clicked() {
                            self.is_exiting = false;
                        }
                    });
                });
        }
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    
    //Run main application window
    eframe::run_native(
        "Martingale Simulator", 
        options, 
        Box::new(|_cc| Box::new(Variables::default()))
    );
}
