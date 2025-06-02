Rust Autonomous Drone (RAD) Flight Controller
================================================================================
This repository implements a flight controller for autonomous drone displays.

Design will explore leveraging a mesh network of the autonomous drones and
distributed computing to increase safety and robustness of the mission.

Vehicle Support
--------------------------------------------------------------------------------
This implementation is designed to support any drone.

[Directly Supported drones](/firmware/drones/README.md)

Monitoring/Analyzing the demonstration
--------------------------------------------------------------------------------
Provides a web-app to monitor the demonstration (both simulated and live).

Provides ground-station logging that records all telemetry and commands.

Provides additional tools to analyze on-vehicle logs.

<!-- Leverages [Cesium](https://cesium.com/) for visualization. -->

Simulation
--------------------------------------------------------------------------------
Provides a simulated physics environment to analyze mission performance under
physical conditions.

Simulation Environment:
* wind - both static and dynamic
* precipitation - rain, snow, hail, etc.
* physical collision
    * static components - buildings, utilities, etc.
    * dynamic components - drones, aircraft, etc.

Simulation Objectives:
* perform the mission
* fail safely (degraded mission performance) to ensure safety
    * avoid collisions
    * land safely (slowly) when vehicle is unable to continue mission objectives

<!-- Leverages [ProjectChrono](https://projectchrono.org/) to provide environmental
physics to the simulated vehicle. -->


