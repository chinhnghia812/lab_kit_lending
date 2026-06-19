# lab_kit_lending

## Project Title
lab_kit_lending

## Project Description
`lab_kit_lending` is a Soroban smart contract that turns a university science department into a shared, transparent lending pool for physical lab equipment — microscopes, Arduino kits, environmental sensors, soldering stations and more. Instead of relying on paper sign-out sheets or a single overworked technician, every checkout, return and overdue event is recorded on the Stellar ledger so students, staff and auditors all see the same source of truth.

## Project Vision
We want every student — especially in under-resourced schools — to have fair, predictable access to the hands-on tools that make STEM education real. By moving the lending ledger on-chain we remove the "lost clipboard" problem, build an objective reputation system for responsible borrowers, and lay the groundwork for inter-campus equipment sharing where a robotics kit owned by University A can be lent to a club at University B with the same trust guarantees as if it never left the room.

## Key Features
- **Admin-curated inventory** — `init` and `list_kit` let the lab manager register physical kits (microscope, sensor pack, etc.) and assign each a unique on-chain `kit_id`.
- **Authenticated borrowing** — `borrow_kit` requires the student to sign with their wallet, locks the kit, and stores a `due_date` so overdue items can be flagged on-chain.
- **Self-service returns** — `return_kit` re-opens the kit for the next borrower, closes the open loan, and reports whether the return was on time.
- **Real-time availability** — `is_available` and `get_kit` give frontends an instant view of the shared catalogue without any off-chain database.
- **Borrower reputation** — `borrower_history` counts every successful return so labs can reward reliable students or gate access to higher-value gear.

## Contract

- **Network:** Stellar Testnet (Public)
- **Scope:** education dApp — see `contracts/lab_kit_lending/src/lib.rs` for the full lab_kit_lending business logic.
- **Functions exposed:** see `Key Features` above and the `pub fn` list in `lib.rs`.
- **Contract ID:** `CDZGQZ5TUBDY7GVR6LKUSFLJ5U2PHJRJY3YX336FH2MTJ3GJJCBXLVC6`
- **Explorer template:** `https://stellar.expert/explorer/testnet/tx/319df94531118ee00a763b66dd87ab946414762b5f4e806597dbace2e15d0245`


## Future Scope
- **Late-return penalties & deposits**: integrate a small XLM / lab-token deposit that is partially withheld when a kit is returned after its `due_date`, automating fair penalties.
- **Cross-campus lending federation**: extend the data model so kits owned by one institution's contract can be safely loaned to students authenticated by another institution.
- **Maintenance & lifecycle tracking**: add `report_damage`, `schedule_service` and end-of-life retirement events so each kit carries a verifiable repair history alongside its borrow history.

## Profile

- **Name:** <!-- Fill github name -->
- **Project:** `lab_kit_lending` (education)
- **Built with:** Soroban SDK 25, Rust, Stellar Testnet
