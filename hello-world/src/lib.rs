#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, symbol_short, Address, log};

#[contracttype]
#[derive(Clone)]
pub struct Loan {
    pub lender: Address,
    pub borrower: Address,
    pub amount: i128,
    pub repaid: bool,
}

const LOAN_COUNT: Symbol = symbol_short!("L_COUNT");

#[contracttype]
pub enum LoanBook {
    Loan(u64),
}

#[contract]
pub struct P2PLendingContract;

#[contractimpl]
impl P2PLendingContract {
    /// Function for a lender to offer a loan to a borrower
    pub fn offer_loan(env: Env, lender: Address, borrower: Address, amount: i128) -> u64 {
        let mut loan_count = env.storage().instance().get(&LOAN_COUNT).unwrap_or(0u64);
        loan_count += 1;

        let loan = Loan {
            lender,
            borrower,
            amount,
            repaid: false,
        };

        env.storage().instance().set(&LoanBook::Loan(loan_count), &loan);
        env.storage().instance().set(&LOAN_COUNT, &loan_count);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Loan Offered: ID {}", loan_count);
        loan_count
    }

    /// Function for a borrower to repay a loan
    pub fn repay_loan(env: Env, caller: Address, loan_id: u64) {
        let mut loan: Loan = env
            .storage()
            .instance()
            .get(&LoanBook::Loan(loan_id))
            .expect("Loan does not exist");

        if loan.borrower != caller {
            panic!("Only borrower can repay the loan");
        }

        if loan.repaid {
            panic!("Loan already repaid");
        }

        loan.repaid = true;
        env.storage().instance().set(&LoanBook::Loan(loan_id), &loan);

        log!(&env, "Loan Repaid: ID {}", loan_id);
    }

    /// Function to view the details of a specific loan
    pub fn view_loan(env: Env, loan_id: u64) -> Loan {
        env.storage()
            .instance()
            .get(&LoanBook::Loan(loan_id))
            .expect("Loan not found")
    }
}
