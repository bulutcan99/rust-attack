Rust-Attack
Rust-Attack is a 2D game written in Rust, featuring strategic planning, resource management, and action-packed turret combat. Players take on the role of a Commander, coordinating turrets to neutralize threats efficiently.

Game Overview
In Rust-Attack, the gameplay revolves around a hierarchical system where Commanders, Turrets, and Anti-Missile entities work together to protect their base. Strategic planning, resource optimization, and quick decision-making are key to success.

Roles and Responsibilities
1. Commander
The Commander oversees strategic operations and ensures the turret performs optimally.
A single Commander can control one turret, focusing on its efficiency and repair when necessary.

Key Responsibilities:

Strategy and Coordination:
Define the overall strategy and coordinate turret operations to maximize efficiency.
Resource Management:
Monitor and manage resources like ammunition and energy for optimal performance.
Turret Repair:
Repair the turret when its health is low or it becomes non-operational.
Example Functions:

Assign targets based on priority:
assign_target(target: Target)
Evaluate threats and decide actions:
evaluate_threats()
Repair the turret when necessary:
repair_turret()
2. Turret
The turret is the frontline defense, responsible for aiming, firing, and maintaining operational readiness. It operates under the guidance of the Commander.

Key Responsibilities:

Target Acquisition and Firing:
Lock onto assigned targets and fire projectiles to neutralize threats.
Angular Limit Control:
Ensure targets are within the turret's predefined firing range.
Operational Health:
Monitor health status and report issues like critical damage or malfunctions.
Out-of-Range Alert:
Notify the Commander if a target is beyond effective range.
Example Functions:

Aim and fire at a given target:
aim_and_fire(target: Target)
Check if a target is within range:
is_target_in_range(target: Target) -> bool
Report health status:
report_health() -> TurretStatus
3. Anti-Missile Entities
Anti-Missile projectiles are launched by turrets to intercept and neutralize incoming threats. Once launched, they operate independently.

Key Responsibilities:

Target Tracking:
Follow the assigned target until neutralized or until the missile expires.
Impact with Target:
Detect and execute an explosion upon colliding with the target.
Physical Movement:
Handle movement mechanics, including speed and direction, to reach the target.
Environmental Interaction:
Detect collisions with other objects and respond accordingly.
Example Functions:

Update position during movement:
update_position(delta_time: f32)
Check for collision with the target:
check_collision(target: Target) -> bool
Trigger an explosion upon impact:
explode()
Features
Hierarchical gameplay with distinct roles for Commanders, Turrets, and Anti-Missiles.
Strategic resource and threat management.
Real-time projectile tracking and environmental interactions.
