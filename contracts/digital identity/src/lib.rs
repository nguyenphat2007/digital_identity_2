#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

// Cấu trúc Hồ sơ người dùng
#[contracttype]
#[derive(Clone)]
pub struct Profile {
    pub name: String,
    pub bio: String,
    pub rep_score: u32, // Điểm uy tín (được người khác vote)
}

#[contract]
pub struct WorkID;

#[contractimpl]
impl WorkID {
    // 1. Tạo hồ sơ
    pub fn create_profile(env: Env, user: Address, name: String, bio: String) {
        user.require_auth(); // Yêu cầu người gọi phải ký giao dịch
        
        let profile = Profile { 
            name, 
            bio, 
            rep_score: 0 
        };
        // Lưu dữ liệu vào blockchain
        env.storage().persistent().set(&user, &profile);
    }

    // 2. Chứng nhận (Endorse / Vote) uy tín cho người khác
    pub fn endorse(env: Env, voter: Address, target: Address) {
        voter.require_auth();
        
        // Không cho phép tự vote cho chính mình
        assert!(voter != target, "Khong the tu endorse cho chinh minh!");
        
        // Lấy profile của người được vote lên
        let mut target_profile: Profile = env.storage().persistent()
            .get(&target)
            .expect("Nguoi dung nay chua co profile");
            
        // Tăng điểm uy tín lên 1
        target_profile.rep_score += 1;
        
        // Lưu cập nhật lại vào blockchain
        env.storage().persistent().set(&target, &target_profile);
    }

    // 3. Xem hồ sơ
    pub fn view_profile(env: Env, user: Address) -> Profile {
        env.storage().persistent().get(&user).expect("Khong tim thay profile")
    }
}