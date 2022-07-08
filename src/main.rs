#![windows_subsystem = "windows"] // hide console window on Windows

use rand::Rng;
use eframe::egui;

struct Variables {
    run: bool,
    done:bool, 

    win_probability: f64,
    balance: f64,
    target: f64,
    
    bet_amount: f64,

    loss_multiplier: f64,
    iterations: usize,

    success_rate:f64,
    bet_cycles: f64,

    can_exit: bool,
    is_exiting: bool,
}
impl Default for Variables {
    fn default() -> Self {
        return Self {
            run: false,
            done: false, 

            win_probability: 50.0,
            balance: 1000.0,
            target: 2000.0,

            bet_amount: 10.0,

            loss_multiplier: 2.0,
            iterations: 1000,

            success_rate: 0.0,
            bet_cycles: 0.0,

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
                    ui.label("Win probability(%):");
                    ui.add(egui::DragValue::new(&mut self.win_probability).speed(1));
                    if self.win_probability < 1.0 {self.win_probability = 1.0;}
                    if self.win_probability > 99.0 {self.win_probability = 99.0;}

                    ui.horizontal(|ui| {

                        ui.label("Account($):");
                        ui.add(egui::DragValue::new(&mut self.balance).speed(10));
                        if self.balance < 1.0 {self.balance = 1.0;}

                        ui.label("Target($):");
                        ui.add(egui::DragValue::new(&mut self.target).speed(10));
                        if self.target < 1.0 {self.target = 1.0;}
                        
                        ui.label("Starting Bet Amount($):");
                        ui.add(egui::DragValue::new(&mut self.bet_amount).speed(1));
                        if self.bet_amount < 1.0 {self.bet_amount = 1.0;}

                    });

                    ui.horizontal(|ui| {
                        ui.label("Bet multiplier on loss:");
                        ui.add(egui::DragValue::new(&mut self.loss_multiplier).speed(0.1));
                        if self.loss_multiplier < 1.0 {self.loss_multiplier = 1.0;}
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

                    let mut outcomes: Vec<bool> = vec![];
                    let mut bets_cycles_for_outcome: Vec<u32> = vec![];

                    let balance_start = self.balance;
                    let bet_start = self.bet_amount;

                    for _ in 0..=self.iterations{

                        let mut bet_cycles:u32 = 0;
                        while self.balance > 0.0 && self.balance < self.target {
            
                            //Random value
                            let roll:f64 = rand::thread_rng().gen_range(0.0..=100.0);

                            //Win
                            if roll <= self.win_probability{

                                self.balance += self.bet_amount;
                                self.bet_amount = bet_start;

                            }

                            //Loss
                            else{
                                self.balance -= self.bet_amount;
                                self.bet_amount *= self.loss_multiplier;
                            }
                            bet_cycles += 1;
                        }
                        
                        
                        if self.balance >= self.target{
                            outcomes.push(true);
                        }
                        else
                        {
                            outcomes.push(false);
                        }

                        bets_cycles_for_outcome.push(bet_cycles);

                        //Reset balance and bet amount each iteration
                        self.balance = balance_start;
                        self.bet_amount = bet_start;
                    }
                    self.success_rate = (outcomes.iter().filter(|&n| *n == true).count() as f64/outcomes.len() as f64) * 100.0;
                    self.bet_cycles = bets_cycles_for_outcome.iter().sum::<u32>() as f64/bets_cycles_for_outcome.len() as f64;
                    self.done = true;
                }
                //Display Results
                else{
                    ui.label("Success Rate: ".to_owned() + &self.success_rate.to_string() + "%");
                    ui.label("Average bets to win/lose: ".to_owned() + &self.bet_cycles.to_string());

                    if ui.button("Reset").clicked(){
                        self.success_rate = 0.0;
                        self.bet_cycles = 0.0;
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