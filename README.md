# **Rust-Attack**

**Rust-Attack** is a 2D game written in Rust, featuring strategic planning, resource management, and action-packed turret combat. Players take on the role of a Commander, coordinating turrets to neutralize threats efficiently.

---

## **Game Overview**

In Rust-Attack, the gameplay revolves around a hierarchical system where **Commanders**, **Turrets**, and **Anti-Missile entities** work together to protect their base. Strategic planning, resource optimization, and quick decision-making are key to success.

---

## **Game Structure**

### **1. Commander**
The Commander oversees strategic operations and ensures the turret performs optimally. Each Commander controls a single turret.

#### **Key Responsibilities:**
- **Strategy and Coordination:**  Define the overall strategy and coordinate turret operations to maximize efficiency.
- **Resource Management:**  Monitor and manage resources like ammunition and energy for optimal performance.
- **Turret Repair:**  Repair the turret when its health is low or it becomes non-operational.

#### **Example Functions:**
```rust
// Assign targets based on priority
fn assign_target(target: Target);

// Evaluate threats and decide actions
fn evaluate_threats();

// Repair the turret when necessary
fn repair_turret();
```

---

### **2. Turret**
The turret is the frontline defense, responsible for aiming, firing, and maintaining operational readiness. It operates under the guidance of the Commander.

#### **Key Responsibilities:**
- **Target Acquisition and Firing:**  Lock onto assigned targets and fire projectiles to neutralize threats.
- **Angular Limit Control:**  Ensure targets are within the turret's predefined firing range.
- **Operational Health:**  Monitor health status and report issues like critical damage or malfunctions.
- **Out-of-Range Alert:**  Notify the Commander if a target is beyond effective range.

#### **Example Functions:**
```rust
// Aim and fire at a given target
fn aim_and_fire(target: Target);

// Check if a target is within range
fn is_target_in_range(target: Target) -> bool;

// Report health status
fn report_health() -> TurretStatus;
```

---

### **3. Anti-Missile Entities**
Anti-Missile projectiles are launched by turrets to intercept and neutralize incoming threats. Once launched, they operate independently.

#### **Key Responsibilities:**
- **Target Tracking:**  Follow the assigned target until neutralized or until the missile expires.
- **Impact with Target:**  Detect and execute an explosion upon colliding with the target.
- **Physical Movement:**  Handle movement mechanics, including speed and direction, to reach the target.
- **Environmental Interaction:**  Detect collisions with other objects and respond accordingly.

#### **Example Functions:**
```rust
// Update position during movement
fn update_position(delta_time: f32);

// Check for collision with the target
fn check_collision(target: Target) -> bool;

// Trigger an explosion upon impact
fn explode();
```

---

## **Features**
- Hierarchical gameplay with distinct roles for Commanders, Turrets, and Anti-Missiles.
- Strategic resource and threat management.
- Real-time projectile tracking and environmental interactions.

---

## **Getting Started**

### **Prerequisites**
- Rust (latest stable version)

### **Installation**
1. Clone the repository:  
   ```bash
   [git clone https://github.com/your-repo/rust-attack.git](https://github.com/bulutcan99/rust-attack.git)
   ```
2. Navigate to the project directory:  
   ```bash
   cd rust-attack
   ```
3. Run the game:  
   ```bash
   cargo run
   ```

---

## **Contributing**
Contributions are welcome! Feel free to open issues or submit pull requests to help improve the game.

---

## **License**
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
