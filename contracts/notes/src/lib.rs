#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, String, Symbol, Vec,
};

/// ======================================================
/// STRUCT POLL
/// ======================================================

#[contracttype]
#[derive(Clone, Debug)]
pub struct Poll {
    id: u64,
    question: String,
    yes_votes: u32,
    no_votes: u32,
    creator: Address,
}

/// ======================================================
/// STORAGE KEY
/// ======================================================

const POLL_DATA: Symbol = symbol_short!("POLL_DATA");

/// ======================================================
/// CONTRACT
/// ======================================================

#[contract]
pub struct PollingContract;

/// ======================================================
/// IMPLEMENTATION
/// ======================================================

#[contractimpl]
impl PollingContract {

    /// ==================================================
    /// GET ALL POLLS
    /// ==================================================
    pub fn get_polls(env: Env) -> Vec<Poll> {
        return env
            .storage()
            .instance()
            .get(&POLL_DATA)
            .unwrap_or(Vec::new(&env));
    }

    /// ==================================================
    /// CREATE POLL
    /// ==================================================
    pub fn create_poll(
        env: Env,
        creator: Address,
        question: String,
    ) -> String {

        // Auth wallet
        creator.require_auth();

        // Ambil data poll lama
        let mut polls: Vec<Poll> = env
            .storage()
            .instance()
            .get(&POLL_DATA)
            .unwrap_or(Vec::new(&env));

        // Buat poll baru
        let poll = Poll {
            id: env.prng().gen::<u64>(),
            question,
            yes_votes: 0,
            no_votes: 0,
            creator,
        };

        // Tambahkan poll
        polls.push_back(poll);

        // Simpan kembali ke storage
        env.storage().instance().set(&POLL_DATA, &polls);

        return String::from_str(&env, "Polling berhasil dibuat");
    }

    /// ==================================================
    /// VOTE YES
    /// ==================================================
    pub fn vote_yes(
        env: Env,
        voter: Address,
        poll_id: u64,
    ) -> String {

        // Auth wallet voter
        voter.require_auth();

        // Ambil data polls
        let mut polls: Vec<Poll> = env
            .storage()
            .instance()
            .get(&POLL_DATA)
            .unwrap_or(Vec::new(&env));

        // Cari poll berdasarkan ID
        for i in 0..polls.len() {

            let mut poll = polls.get(i).unwrap();

            if poll.id == poll_id {

                // Tambah vote yes
                poll.yes_votes += 1;

                // Replace data lama
                polls.set(i, poll);

                // Save ulang ke storage
                env.storage().instance().set(&POLL_DATA, &polls);

                return String::from_str(&env, "Vote YES berhasil");
            }
        }

        return String::from_str(&env, "Poll tidak ditemukan");
    }

    /// ==================================================
    /// VOTE NO
    /// ==================================================
    pub fn vote_no(
        env: Env,
        voter: Address,
        poll_id: u64,
    ) -> String {

        // Auth wallet voter
        voter.require_auth();

        // Ambil data polls
        let mut polls: Vec<Poll> = env
            .storage()
            .instance()
            .get(&POLL_DATA)
            .unwrap_or(Vec::new(&env));

        // Cari poll berdasarkan ID
        for i in 0..polls.len() {

            let mut poll = polls.get(i).unwrap();

            if poll.id == poll_id {

                // Tambah vote no
                poll.no_votes += 1;

                // Replace data lama
                polls.set(i, poll);

                // Save ulang ke storage
                env.storage().instance().set(&POLL_DATA, &polls);

                return String::from_str(&env, "Vote NO berhasil");
            }
        }

        return String::from_str(&env, "Poll tidak ditemukan");
    }

    /// ==================================================
    /// DELETE POLL
    /// ==================================================
    pub fn delete_poll(
        env: Env,
        creator: Address,
        poll_id: u64,
    ) -> String {

        // Auth wallet creator
        creator.require_auth();

        // Ambil data polls
        let mut polls: Vec<Poll> = env
            .storage()
            .instance()
            .get(&POLL_DATA)
            .unwrap_or(Vec::new(&env));

        // Cari poll berdasarkan ID
        for i in 0..polls.len() {

            let poll = polls.get(i).unwrap();

            if poll.id == poll_id {

                // Hanya creator yang boleh hapus
                if poll.creator != creator {
                    return String::from_str(
                        &env,
                        "Bukan pembuat polling",
                    );
                }

                // Hapus poll
                polls.remove(i);

                // Save ulang
                env.storage().instance().set(&POLL_DATA, &polls);

                return String::from_str(
                    &env,
                    "Polling berhasil dihapus",
                );
            }
        }

        return String::from_str(&env, "Poll tidak ditemukan");
    }
}

mod test;