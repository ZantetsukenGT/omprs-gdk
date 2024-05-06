use omp_codegen::native;

use std::ffi::c_void;

use crate::{
    players::Player,
    types::vector::{Vector3, Vector4},
};

use super::{Vehicle, VehicleParams};

native!(CreateVehicle, modelid: isize, pos: Vector3, rotation: f32, colour1: isize, colour2: isize, respawnDelay: isize, addSiren: bool, -> struct Vehicle);
native!(GetVehicleSeats, modelid: isize,-> isize);
native!(DestroyVehicle, vehicle: struct Vehicle);
native!(IsVehicleStreamedIn, vehicle: struct Vehicle, player: struct Player, -> bool);
native!(GetVehiclePos, vehicle: struct Vehicle, pos: mut Vector3);
native!(SetVehiclePos, vehicle: struct Vehicle, pos: Vector3);
native!(GetVehicleZAngle, vehicle: struct Vehicle,-> f32);
native!(GetVehicleRotationQuat, vehicle: struct Vehicle, rotation: mut Vector4);
native!(GetVehicleDistanceFromPoint, vehicle: struct Vehicle, pos: Vector3,-> f32);
native!(SetVehicleZAngle, vehicle: struct Vehicle, angle: f32);
native!(SetVehicleParamsForPlayer, vehicle: struct Vehicle, player: struct Player, params: VehicleParams );
native!(SetManualVehicleEngineAndLights, set: bool);
native!(SetVehicleParams, vehicle: struct Vehicle, params: VehicleParams);
native!(GetVehicleParams, vehicle: struct Vehicle, params: mut VehicleParams);
native!(SetVehicleToRespawn, vehicle: struct Vehicle);
native!(LinkVehicleToInterior, vehicle: struct Vehicle, interiorid: isize);
native!(AddVehicleComponent, vehicle: struct Vehicle, componentid: isize);
native!(RemoveVehicleComponent, vehicle: struct Vehicle, componentid: isize);
native!(ChangeVehicleColor, vehicle: struct Vehicle, colour1: isize, colour2: isize);
native!(ChangeVehiclePaintjob, vehicle: struct Vehicle, paintjobid: isize);
native!(SetVehicleHealth, vehicle: struct Vehicle, health: f32);
native!(GetVehicleHealth, vehicle: struct Vehicle,-> f32);
native!(AttachTrailerToVehicle, vehicle: struct Vehicle,trailer: struct Vehicle);
native!(DetachTrailerFromVehicle, vehicle: struct Vehicle);
native!(IsTrailerAttachedToVehicle, vehicle: struct Vehicle, -> bool);
native!(GetVehicleTrailer, vehicle: struct Vehicle, -> struct Vehicle);
native!(SetVehicleNumberPlate, vehicle: struct Vehicle, numberPlate: str);
native!(GetVehicleModel, vehicle: struct Vehicle,-> isize);
native!(GetVehicleComponentInSlot, vehicle: struct Vehicle, slot: isize,-> isize);
native!(GetVehicleComponentType, componentid: isize,-> isize);
native!(VehicleCanHaveComponent, modelid: isize, componentid: isize, -> bool);
native!(GetRandomCarColPair, modelid: isize, colour1:mut isize, colour2: mut isize, colour3: mut isize, colour4: mut isize);
native!(CarColIndexToColour, colourIndex: isize, alpha: isize,-> isize);
native!(RepairVehicle, vehicle: struct Vehicle);
native!(GetVehicleVelocity, vehicle: struct Vehicle, velocity: mut Vector3);
native!(SetVehicleVelocity, vehicle: struct Vehicle, velocity: Vector3);
native!(SetVehicleAngularVelocity, vehicle: struct Vehicle, velocity: Vector3);
native!(GetVehicleDamageStatus, vehicle: struct Vehicle, panels: mut isize, doors: mut isize, lights: mut isize, tires: mut isize);
native!(UpdateVehicleDamageStatus, vehicle: struct Vehicle, panels: isize, doors: isize, lights: isize, tires: isize);
native!(GetVehicleModelInfo, vehiclemodel: isize, infotype: isize, pos: mut Vector3);
native!(SetVehicleVirtualWorld, vehicle: struct Vehicle, virtualWorld: isize);
native!(GetVehicleVirtualWorld, vehicle: struct Vehicle,-> isize);
native!(GetVehicleLandingGearState, vehicle: struct Vehicle,-> isize);
native!(AddStaticVehicle, modelid: isize, spawn: Vector3, angle: f32, colour1: isize, colour2: isize, respawnDelay: isize,addSiren: bool, -> struct Vehicle);
native!(EnableVehicleFriendlyFire, set: bool);
native!(GetVehicleSpawnInfo, vehicle: struct Vehicle, position: mut Vector3, rotation: mut f32, colour1: mut isize, colour2: mut isize, respawnDelay: mut isize, modelID: mut isize,siren: mut bool, interior: mut isize);
native!(SetVehicleSpawnInfo, vehicle: struct Vehicle, modelid: isize, position: Vector3, rotation: f32, colour1: isize, colour2: isize, respawn_time: isize, interior: isize);
native!(GetVehicleModelCount, modelid: isize,-> isize);
native!(GetVehicleModelsUsed,-> isize);
native!(GetVehiclePaintjob, vehicle: struct Vehicle,-> isize);
native!(GetVehicleColor, vehicle: struct Vehicle, colour1: mut isize, colour2: mut isize);
native!(GetVehicleInterior, vehicle: struct Vehicle,-> isize);
native!(GetVehicleNumberPlate, vehicle: struct Vehicle, number_plate: mut str);
native!(SetVehicleRespawnDelay, vehicle: struct Vehicle, respawn_delay: isize);
native!(GetVehicleRespawnDelay, vehicle: struct Vehicle,-> isize);
native!(GetVehicleCab, vehicle: struct Vehicle, -> struct Vehicle);
native!(GetVehicleOccupiedTick, vehicle: struct Vehicle,-> isize);
native!(GetVehicleRespawnTick, vehicle: struct Vehicle,-> isize);
native!(HasVehicleBeenOccupied, vehicle: struct Vehicle, -> bool);
native!(IsVehicleOccupied, vehicle: struct Vehicle, -> bool);
native!(IsVehicleDead, vehicle: struct Vehicle, -> bool);
native!(ToggleVehicleSirenEnabled, vehicle: struct Vehicle, status: bool);
native!(IsVehicleSirenEnabled, vehicle: struct Vehicle, -> bool);
native!(GetVehicleLastDriver, vehicle: struct Vehicle,-> isize);
native!(GetVehicleDriver, vehicle: struct Vehicle, -> struct Player);
native!(IsPlayerInModShop, player: struct Player, -> bool);
native!(GetPlayerSirenState, player: struct Player,-> isize);
native!(GetPlayerLandingGearState, player: struct Player,-> isize);
native!(GetPlayerHydraReactorAngle, player: struct Player,-> isize);
native!(GetPlayerTrainSpeed, player: struct Player,-> f32);
native!(GetVehicleSirenState, vehicle: struct Vehicle,-> isize);
native!(GetVehicleHydraReactorAngle, vehicle: struct Vehicle,-> isize);
native!(GetVehicleTrainSpeed, vehicle: struct Vehicle,-> f32);
native!(GetVehicleMatrix, vehicle: struct Vehicle, right: mut Vector3, up: mut Vector3, at: mut Vector3);
native!(GetVehicleOccupant, vehicle: struct Vehicle, seat: isize, -> struct Player);
native!(GetVehicleMaxPassengers, model: isize,-> isize);
native!(CountVehicleOccupants, vehicle: struct Vehicle,-> isize);
native!(GetVehicleFromID, id: isize, -> struct Vehicle);
native!(GetVehicleID, vehicle: struct Vehicle,-> isize);

pub fn load_functions() {
    load_function!(CreateVehicle);
    load_function!(GetVehicleSeats);
    load_function!(DestroyVehicle);
    load_function!(IsVehicleStreamedIn);
    load_function!(GetVehiclePos);
    load_function!(SetVehiclePos);
    load_function!(GetVehicleZAngle);
    load_function!(GetVehicleRotationQuat);
    load_function!(GetVehicleDistanceFromPoint);
    load_function!(SetVehicleZAngle);
    load_function!(SetVehicleParamsForPlayer);
    load_function!(SetManualVehicleEngineAndLights);
    load_function!(SetVehicleParams);
    load_function!(GetVehicleParams);
    load_function!(SetVehicleToRespawn);
    load_function!(LinkVehicleToInterior);
    load_function!(AddVehicleComponent);
    load_function!(RemoveVehicleComponent);
    load_function!(ChangeVehicleColor);
    load_function!(ChangeVehiclePaintjob);
    load_function!(SetVehicleHealth);
    load_function!(GetVehicleHealth);
    load_function!(AttachTrailerToVehicle);
    load_function!(DetachTrailerFromVehicle);
    load_function!(IsTrailerAttachedToVehicle);
    load_function!(GetVehicleTrailer);
    load_function!(SetVehicleNumberPlate);
    load_function!(GetVehicleModel);
    load_function!(GetVehicleComponentInSlot);
    load_function!(GetVehicleComponentType);
    load_function!(VehicleCanHaveComponent);
    load_function!(GetRandomCarColPair);
    load_function!(CarColIndexToColour);
    load_function!(RepairVehicle);
    load_function!(GetVehicleVelocity);
    load_function!(SetVehicleVelocity);
    load_function!(SetVehicleAngularVelocity);
    load_function!(GetVehicleDamageStatus);
    load_function!(UpdateVehicleDamageStatus);
    load_function!(GetVehicleModelInfo);
    load_function!(SetVehicleVirtualWorld);
    load_function!(GetVehicleVirtualWorld);
    load_function!(GetVehicleLandingGearState);
    load_function!(AddStaticVehicle);
    load_function!(EnableVehicleFriendlyFire);
    load_function!(GetVehicleSpawnInfo);
    load_function!(SetVehicleSpawnInfo);
    load_function!(GetVehicleModelCount);
    load_function!(GetVehicleModelsUsed);
    load_function!(GetVehiclePaintjob);
    load_function!(GetVehicleColor);
    load_function!(GetVehicleInterior);
    load_function!(GetVehicleNumberPlate);
    load_function!(SetVehicleRespawnDelay);
    load_function!(GetVehicleRespawnDelay);
    load_function!(GetVehicleCab);
    load_function!(GetVehicleOccupiedTick);
    load_function!(GetVehicleRespawnTick);
    load_function!(HasVehicleBeenOccupied);
    load_function!(IsVehicleOccupied);
    load_function!(IsVehicleDead);
    load_function!(ToggleVehicleSirenEnabled);
    load_function!(IsVehicleSirenEnabled);
    load_function!(GetVehicleLastDriver);
    load_function!(GetVehicleDriver);
    load_function!(IsPlayerInModShop);
    load_function!(GetPlayerSirenState);
    load_function!(GetPlayerLandingGearState);
    load_function!(GetPlayerHydraReactorAngle);
    load_function!(GetPlayerTrainSpeed);
    load_function!(GetVehicleSirenState);
    load_function!(GetVehicleHydraReactorAngle);
    load_function!(GetVehicleTrainSpeed);
    load_function!(GetVehicleMatrix);
    load_function!(GetVehicleOccupant);
    load_function!(GetVehicleMaxPassengers);
    load_function!(CountVehicleOccupants);
    load_function!(GetVehicleFromID);
    load_function!(GetVehicleID);
}