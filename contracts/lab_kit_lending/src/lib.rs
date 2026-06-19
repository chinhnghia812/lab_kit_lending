#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Map, String, Symbol};

/// Storage keys used by the LabKitLending contract.
const ADMIN: Symbol = symbol_short!("ADMIN");
const KITS: Symbol = symbol_short!("KITS");
const LOANS: Symbol = symbol_short!("LOANS");
const HISTORY: Symbol = symbol_short!("HISTORY");

/// On-chain record describing a physical lab kit (microscope, sensor set,
/// soldering kit, etc.) that lives in the shared lending pool.
#[contracttype]
#[derive(Clone)]
pub struct Kit {
    pub owner: Address,
    pub name: String,
    pub available: bool,
    pub total_borrows: u32,
}

/// Open loan record describing who is currently holding a kit and when it
/// is due back to the lab.
#[contracttype]
#[derive(Clone)]
pub struct Loan {
    pub borrower: Address,
    pub kit_id: u32,
    pub borrowed_at: u64,
    pub due_date: u64,
}

#[contract]
pub struct LabKitLending;

#[contractimpl]
impl LabKitLending {
    /// Initialize the lending program with the lab administrator (e.g. the
    /// teacher / lab manager) who is allowed to register new kits.
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&ADMIN) {
            panic!("already initialized");
        }
        admin.require_auth();
        env.storage().instance().set(&ADMIN, &admin);
    }

    /// Register a new lab kit in the shared inventory. Only the admin may
    /// list kits. Returns the freshly assigned `kit_id`.
    pub fn list_kit(env: Env, owner: Address, name: String) -> u32 {
        let admin: Address = env
            .storage()
            .instance()
            .get(&ADMIN)
            .expect("contract not initialized");
        admin.require_auth();

        let mut kits: Map<u32, Kit> = env
            .storage()
            .persistent()
            .get(&KITS)
            .unwrap_or(Map::new(&env));

        let kit_id: u32 = kits.len() as u32 + 1;
        let kit = Kit {
            owner,
            name,
            available: true,
            total_borrows: 0,
        };
        kits.set(kit_id, kit);
        env.storage().persistent().set(&KITS, &kits);
        kit_id
    }

    /// A student borrows an available kit and commits to a `due_date`
    /// (unix-seconds ledger timestamp). The borrower must authorize the
    /// call so the contract knows the loan is genuine.
    pub fn borrow_kit(env: Env, borrower: Address, kit_id: u32, due_date: u64) {
        borrower.require_auth();

        let mut kits: Map<u32, Kit> = env
            .storage()
            .persistent()
            .get(&KITS)
            .expect("no kits registered yet");
        let mut kit: Kit = kits.get(kit_id).expect("kit does not exist");

        if !kit.available {
            panic!("kit currently on loan");
        }
        let now = env.ledger().timestamp();
        if due_date <= now {
            panic!("due_date must be in the future");
        }

        kit.available = false;
        kit.total_borrows += 1;
        kits.set(kit_id, kit);
        env.storage().persistent().set(&KITS, &kits);

        let mut loans: Map<u32, Loan> = env
            .storage()
            .persistent()
            .get(&LOANS)
            .unwrap_or(Map::new(&env));
        loans.set(
            kit_id,
            Loan {
                borrower: borrower.clone(),
                kit_id,
                borrowed_at: now,
                due_date,
            },
        );
        env.storage().persistent().set(&LOANS, &loans);
    }

    /// Return a previously borrowed kit. Marks the kit available again,
    /// closes the open loan, and increments the borrower's lifetime
    /// history counter for reputation purposes.
    pub fn return_kit(env: Env, borrower: Address, kit_id: u32) -> bool {
        borrower.require_auth();

        let mut loans: Map<u32, Loan> = env
            .storage()
            .persistent()
            .get(&LOANS)
            .expect("no active loans");
        let loan: Loan = loans.get(kit_id).expect("kit is not on loan");
        if loan.borrower != borrower {
            panic!("only the original borrower can return this kit");
        }

        let mut kits: Map<u32, Kit> = env
            .storage()
            .persistent()
            .get(&KITS)
            .expect("no kits registered");
        let mut kit: Kit = kits.get(kit_id).expect("kit does not exist");
        kit.available = true;
        kits.set(kit_id, kit);
        env.storage().persistent().set(&KITS, &kits);

        loans.remove(kit_id);
        env.storage().persistent().set(&LOANS, &loans);

        let mut history: Map<Address, u32> = env
            .storage()
            .persistent()
            .get(&HISTORY)
            .unwrap_or(Map::new(&env));
        let prev = history.get(borrower.clone()).unwrap_or(0);
        history.set(borrower, prev + 1);
        env.storage().persistent().set(&HISTORY, &history);

        let on_time = env.ledger().timestamp() <= loan.due_date;
        on_time
    }

    /// Read-only helper: returns `true` if the kit is currently free to
    /// borrow, `false` if checked out or unknown.
    pub fn is_available(env: Env, kit_id: u32) -> bool {
        let kits: Map<u32, Kit> = env
            .storage()
            .persistent()
            .get(&KITS)
            .unwrap_or(Map::new(&env));
        match kits.get(kit_id) {
            Some(kit) => kit.available,
            None => false,
        }
    }

    /// Read-only helper: how many kits has this borrower successfully
    /// returned in total? Useful for trust / loyalty scoring inside the
    /// university lab.
    pub fn borrower_history(env: Env, borrower: Address) -> u32 {
        let history: Map<Address, u32> = env
            .storage()
            .persistent()
            .get(&HISTORY)
            .unwrap_or(Map::new(&env));
        history.get(borrower).unwrap_or(0)
    }

    /// Read-only helper: fetch full kit metadata so a frontend / catalogue
    /// page can render the kit card without extra calls.
    pub fn get_kit(env: Env, kit_id: u32) -> Kit {
        let kits: Map<u32, Kit> = env
            .storage()
            .persistent()
            .get(&KITS)
            .expect("no kits registered");
        kits.get(kit_id).expect("kit does not exist")
    }
}
