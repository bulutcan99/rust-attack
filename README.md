# rust-attack

2D game written in Rust

1. Commander
   The Commander is responsible for strategic planning, resource management, and turret coordination. Each Commander can control a single turret. The Commander ensures that the turret operates efficiently and stays functional.

Responsibilities:
Strategy and Coordination:
The Commander defines the game's overall strategy and coordinates the turret's operations to maximize efficiency.
Resource Management:
Monitors and manages resources such as ammunition and energy to ensure optimal performance.
Turret Repair:
Repairs the turret when its health is low or it becomes non-operational.
Example Functions:
Assign targets to the turret based on priority.
assign_target(target: Target)
Evaluate threats and decide the next action.
evaluate_threats()
Repair the turret when necessary.
repair_turret() 2. Turret
The Turret is responsible for aiming, firing, and maintaining its operational state. It interacts closely with the Commander to receive targets and operate within its defined limits.

Responsibilities:
Target Acquisition and Firing:
Locks onto the assigned target and fires projectiles to neutralize threats.
Angular Limit Control:
Ensures the target is within the turret's predefined angular firing range before engaging.
Operational Health:
Monitors its health status and reports any issues, such as critical damage or malfunction.
Out-of-Range Alert:
Notifies the Commander if a target is beyond the turret's effective range.
Example Functions:
Aim and shoot at the given target.
aim_and_fire(target: Target)
Check if the target is within range.
is_target_in_range(target: Target) -> bool
Report its current health status.
report_health() -> TurretStatus 3. Anti-Missile
Anti-Missile entities are projectiles launched by the turret to intercept and neutralize incoming threats. They operate independently once fired.

Responsibilities:
Target Tracking:
Tracks and follows the assigned target until it is neutralized or the missile expires.
Impact with Target:
Detects and executes an explosion upon colliding with the target.
Physical Movement:
Handles its own movement mechanics, including speed and direction, to reach the target.
Environmental Interaction:
Detects collisions with other objects in the environment and reacts accordingly.
Example Functions:
Update position during movement.
update_position(delta_time: f32)
Check for a collision with the target.
check_collision(target: Target) -> bool
Trigger an explosion upon impact.
explode()
