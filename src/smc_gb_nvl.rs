use std::collections::HashMap;

use serde_json::Value;

use crate::nvidia_gbx00::Bmc as GBX00Bmc;
use crate::standard::RedfishStandard;
use crate::{MachineSetupDiff, MachineSetupStatus, Redfish, RedfishError};

pub struct Bmc {
    s: RedfishStandard,
    g: GBX00Bmc,
}

impl Bmc {
    pub fn new(s: RedfishStandard) -> Result<Self, RedfishError> {
        let g = GBX00Bmc::new(s.clone())?;
        Ok(Self { s, g })
    }
}

impl Redfish for Bmc {
    fn change_username<'a>(
        &'a self,
        old_name: &'a str,
        new_name: &'a str,
    ) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.change_username(old_name, new_name)
    }

    fn change_password<'a>(
        &'a self,
        username: &'a str,
        new_pass: &'a str,
    ) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.change_password(username, new_pass)
    }

    fn change_password_by_id<'a>(
        &'a self,
        account_id: &'a str,
        new_pass: &'a str,
    ) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.change_password_by_id(account_id, new_pass)
    }

    fn get_accounts<'a>(
        &'a self,
    ) -> crate::RedfishFuture<
        'a,
        Result<Vec<crate::model::account_service::ManagerAccount>, RedfishError>,
    > {
        self.g.get_accounts()
    }

    fn create_user<'a>(
        &'a self,
        username: &'a str,
        password: &'a str,
        role_id: crate::RoleId,
    ) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.create_user(username, password, role_id)
    }

    fn delete_user<'a>(
        &'a self,
        username: &'a str,
    ) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.delete_user(username)
    }

    fn get_firmware<'a>(
        &'a self,
        id: &'a str,
    ) -> crate::RedfishFuture<
        'a,
        Result<crate::model::software_inventory::SoftwareInventory, RedfishError>,
    > {
        self.g.get_firmware(id)
    }

    fn get_software_inventories<'a>(
        &'a self,
    ) -> crate::RedfishFuture<'a, Result<Vec<String>, RedfishError>> {
        self.g.get_software_inventories()
    }

    fn get_tasks<'a>(&'a self) -> crate::RedfishFuture<'a, Result<Vec<String>, RedfishError>> {
        self.g.get_tasks()
    }

    fn get_task<'a>(
        &'a self,
        id: &'a str,
    ) -> crate::RedfishFuture<'a, Result<crate::model::task::Task, RedfishError>> {
        self.g.get_task(id)
    }

    fn get_power_state<'a>(
        &'a self,
    ) -> crate::RedfishFuture<'a, Result<crate::PowerState, RedfishError>> {
        self.g.get_power_state()
    }

    fn get_service_root<'a>(
        &'a self,
    ) -> crate::RedfishFuture<'a, Result<crate::model::service_root::ServiceRoot, RedfishError>>
    {
        self.g.get_service_root()
    }

    fn get_systems<'a>(&'a self) -> crate::RedfishFuture<'a, Result<Vec<String>, RedfishError>> {
        self.g.get_systems()
    }

    fn get_system<'a>(
        &'a self,
    ) -> crate::RedfishFuture<'a, Result<crate::model::ComputerSystem, RedfishError>> {
        self.g.get_system()
    }

    fn get_managers<'a>(&'a self) -> crate::RedfishFuture<'a, Result<Vec<String>, RedfishError>> {
        self.g.get_managers()
    }

    fn get_manager<'a>(
        &'a self,
    ) -> crate::RedfishFuture<'a, Result<crate::model::Manager, RedfishError>> {
        self.g.get_manager()
    }

    fn get_secure_boot<'a>(
        &'a self,
    ) -> crate::RedfishFuture<'a, Result<crate::model::secure_boot::SecureBoot, RedfishError>> {
        self.g.get_secure_boot()
    }

    fn disable_secure_boot<'a>(&'a self) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.disable_secure_boot()
    }

    fn enable_secure_boot<'a>(&'a self) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.enable_secure_boot()
    }

    fn get_secure_boot_certificate<'a>(
        &'a self,
        database_id: &'a str,
        certificate_id: &'a str,
    ) -> crate::RedfishFuture<'a, Result<crate::model::certificate::Certificate, RedfishError>>
    {
        self.g
            .get_secure_boot_certificate(database_id, certificate_id)
    }

    fn get_secure_boot_certificates<'a>(
        &'a self,
        database_id: &'a str,
    ) -> crate::RedfishFuture<'a, Result<Vec<String>, RedfishError>> {
        self.g.get_secure_boot_certificates(database_id)
    }

    fn add_secure_boot_certificate<'a>(
        &'a self,
        pem_cert: &'a str,
        database_id: &'a str,
    ) -> crate::RedfishFuture<'a, Result<crate::model::task::Task, RedfishError>> {
        self.g.add_secure_boot_certificate(pem_cert, database_id)
    }

    fn get_power_metrics<'a>(
        &'a self,
    ) -> crate::RedfishFuture<'a, Result<crate::model::power::Power, RedfishError>> {
        self.g.get_power_metrics()
    }

    fn power<'a>(
        &'a self,
        action: crate::SystemPowerControl,
    ) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.power(action)
    }

    fn bmc_reset<'a>(&'a self) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.bmc_reset()
    }

    fn chassis_reset<'a>(
        &'a self,
        chassis_id: &'a str,
        reset_type: crate::SystemPowerControl,
    ) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.chassis_reset(chassis_id, reset_type)
    }

    fn bmc_reset_to_defaults<'a>(&'a self) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.bmc_reset_to_defaults()
    }

    fn get_thermal_metrics<'a>(
        &'a self,
    ) -> crate::RedfishFuture<'a, Result<crate::model::thermal::Thermal, RedfishError>> {
        self.g.get_thermal_metrics()
    }

    fn get_gpu_sensors<'a>(
        &'a self,
    ) -> crate::RedfishFuture<'a, Result<Vec<crate::model::sensor::GPUSensors>, RedfishError>> {
        self.g.get_gpu_sensors()
    }

    fn get_system_event_log<'a>(
        &'a self,
    ) -> crate::RedfishFuture<'a, Result<Vec<crate::model::sel::LogEntry>, RedfishError>> {
        self.g.get_system_event_log()
    }

    fn get_bmc_event_log<'a>(
        &'a self,
        from: Option<chrono::DateTime<chrono::Utc>>,
    ) -> crate::RedfishFuture<'a, Result<Vec<crate::model::sel::LogEntry>, RedfishError>> {
        self.g.get_bmc_event_log(from)
    }

    fn get_drives_metrics<'a>(
        &'a self,
    ) -> crate::RedfishFuture<'a, Result<Vec<crate::model::storage::Drives>, RedfishError>> {
        self.g.get_drives_metrics()
    }

    fn machine_setup<'a>(
        &'a self,
        _boot_interface: Option<crate::BootInterfaceRef<'a>>,
        _bios_profiles: &'a crate::BiosProfileVendor,
        _selected_profile: crate::BiosProfileType,
        _oem_manager_profiles: &'a crate::BiosProfileVendor,
    ) -> crate::RedfishFuture<'a, Result<Option<String>, RedfishError>> {
        Box::pin(async move {
            self.disable_secure_boot().await?;
            let attributes = self.s.bios_attributes().await?.as_object().map(|e| {
                e.iter()
                    .filter_map(|(k, _v)| {
                        if k.contains("Pcie6DisableOptionROM") {
                            Some((k.to_string(), false.into()))
                        } else {
                            None
                        }
                    })
                    .collect::<HashMap<String, Value>>()
            });

            if let Some(attrs) = attributes.filter(|a| !a.is_empty()) {
                let body = HashMap::from([("Attributes", attrs)]);
                let url = format!("Systems/{}/Bios/Settings", self.s.system_id());
                self.s
                    .client
                    .patch(&url, body)
                    .await
                    .map(|_status_code| None)
            } else {
                Ok(None)
            }
        })
    }

    fn machine_setup_status<'a>(
        &'a self,
        boot_interface: Option<crate::BootInterfaceRef<'a>>,
    ) -> crate::RedfishFuture<'a, Result<crate::MachineSetupStatus, RedfishError>> {
        Box::pin(async move {
            // Resolve `InterfaceId` to a MAC via the Redfish-standard
            // EthernetInterface resource.
            let resolved_mac = match boot_interface {
                Some(b) => Some(crate::resolve_boot_interface_mac(self, b).await?),
                None => None,
            };
            let boot_interface_mac = resolved_mac.as_deref();

            // Check BIOS and BMC attributes
            let mut diffs = Vec::new();
            let sb = self.get_secure_boot().await?;
            if sb.secure_boot_enable.unwrap_or(false) {
                diffs.push(MachineSetupDiff {
                    key: "SecureBoot".to_string(),
                    expected: "false".to_string(),
                    actual: "true".to_string(),
                });
            }
            if let Some(obj) = self.s.bios_attributes().await?.as_object() {
                for (k, v) in obj {
                    if k.contains("Pcie6DisableOptionROM") && v.as_bool() != Some(false) {
                        diffs.push(crate::MachineSetupDiff {
                            key: k.clone(),
                            expected: "false".into(),
                            actual: v.to_string(),
                        });
                    }
                }
            }

            // Check the first boot option
            if let Some(mac) = boot_interface_mac {
                let (expected, actual) = self
                    .g
                    .get_expected_and_actual_first_boot_option(mac)
                    .await?;
                if expected.is_none() || expected != actual {
                    diffs.push(MachineSetupDiff {
                        key: "boot_first".to_string(),
                        expected: expected.unwrap_or_else(|| "Not found".to_string()),
                        actual: actual.unwrap_or_else(|| "Not found".to_string()),
                    });
                }
            }

            Ok(MachineSetupStatus {
                is_done: diffs.is_empty(),
                diffs,
            })
        })
    }

    fn is_bios_setup<'a>(
        &'a self,
        _boot_interface: Option<crate::BootInterfaceRef<'a>>,
    ) -> crate::RedfishFuture<'a, Result<bool, RedfishError>> {
        Box::pin(async move { 
            // Check BIOS and BMC attributes
            let mut diffs = Vec::new();
            let sb = self.get_secure_boot().await?;
            if sb.secure_boot_enable.unwrap_or(false) {
                diffs.push(MachineSetupDiff {
                    key: "SecureBoot".to_string(),
                    expected: "false".to_string(),
                    actual: "true".to_string(),
                });
            }
            if let Some(obj) = self.s.bios_attributes().await?.as_object() {
                for (k, v) in obj {
                    if k.contains("Pcie6DisableOptionROM") && v.as_bool() != Some(false) {
                        diffs.push(crate::MachineSetupDiff {
                            key: k.clone(),
                            expected: "false".into(),
                            actual: v.to_string(),
                        });
                    }
                }
            }

            Ok(diffs.is_empty()) 
        })
    }

    fn set_machine_password_policy<'a>(
        &'a self,
    ) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.set_machine_password_policy()
    }

    fn lockdown<'a>(
        &'a self,
        target: crate::EnabledDisabled,
    ) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.lockdown(target)
    }

    fn lockdown_status<'a>(
        &'a self,
    ) -> crate::RedfishFuture<'a, Result<crate::Status, RedfishError>> {
        self.g.lockdown_status()
    }

    fn setup_serial_console<'a>(&'a self) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.setup_serial_console()
    }

    fn serial_console_status<'a>(
        &'a self,
    ) -> crate::RedfishFuture<'a, Result<crate::Status, RedfishError>> {
        self.g.serial_console_status()
    }

    fn get_boot_options<'a>(
        &'a self,
    ) -> crate::RedfishFuture<'a, Result<crate::BootOptions, RedfishError>> {
        self.g.get_boot_options()
    }

    fn get_boot_option<'a>(
        &'a self,
        option_id: &'a str,
    ) -> crate::RedfishFuture<'a, Result<crate::model::BootOption, RedfishError>> {
        self.g.get_boot_option(option_id)
    }

    fn boot_once<'a>(
        &'a self,
        target: crate::Boot,
    ) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.boot_once(target)
    }

    fn boot_first<'a>(
        &'a self,
        target: crate::Boot,
    ) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.boot_first(target)
    }

    fn set_boot_override<'a>(
        &'a self,
        settings: crate::BootOverride,
    ) -> crate::RedfishFuture<'a, Result<Option<String>, RedfishError>> {
        self.g.set_boot_override(settings)
    }

    fn change_boot_order<'a>(
        &'a self,
        boot_array: Vec<String>,
    ) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.change_boot_order(boot_array)
    }

    fn clear_tpm<'a>(&'a self) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.clear_tpm()
    }

    fn pcie_devices<'a>(
        &'a self,
    ) -> crate::RedfishFuture<'a, Result<Vec<crate::PCIeDevice>, RedfishError>> {
        self.g.pcie_devices()
    }

    fn update_firmware<'a>(
        &'a self,
        filename: tokio::fs::File,
    ) -> crate::RedfishFuture<'a, Result<crate::model::task::Task, RedfishError>> {
        self.g.update_firmware(filename)
    }

    fn update_firmware_multipart<'a>(
        &'a self,
        firmware: &'a std::path::Path,
        reboot: bool,
        timeout: std::time::Duration,
        component_type: crate::model::update_service::ComponentType,
    ) -> crate::RedfishFuture<'a, Result<String, RedfishError>> {
        self.g
            .update_firmware_multipart(firmware, reboot, timeout, component_type)
    }

    fn update_firmware_simple_update<'a>(
        &'a self,
        image_uri: &'a str,
        targets: Vec<String>,
        transfer_protocol: crate::model::update_service::TransferProtocolType,
    ) -> crate::RedfishFuture<'a, Result<crate::model::task::Task, RedfishError>> {
        self.g
            .update_firmware_simple_update(image_uri, targets, transfer_protocol)
    }

    fn bios<'a>(
        &'a self,
    ) -> crate::RedfishFuture<
        'a,
        Result<std::collections::HashMap<String, serde_json::Value>, RedfishError>,
    > {
        self.g.bios()
    }

    fn set_bios<'a>(
        &'a self,
        values: std::collections::HashMap<String, serde_json::Value>,
    ) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.set_bios(values)
    }

    fn reset_bios<'a>(&'a self) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.reset_bios()
    }

    fn pending<'a>(
        &'a self,
    ) -> crate::RedfishFuture<
        'a,
        Result<std::collections::HashMap<String, serde_json::Value>, RedfishError>,
    > {
        self.g.pending()
    }

    fn clear_pending<'a>(&'a self) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.clear_pending()
    }

    fn get_network_device_functions<'a>(
        &'a self,
        chassis_id: &'a str,
    ) -> crate::RedfishFuture<'a, Result<Vec<String>, RedfishError>> {
        self.g.get_network_device_functions(chassis_id)
    }

    fn get_network_device_function<'a>(
        &'a self,
        chassis_id: &'a str,
        id: &'a str,
        port: Option<&'a str>,
    ) -> crate::RedfishFuture<'a, Result<crate::NetworkDeviceFunction, RedfishError>> {
        self.g.get_network_device_function(chassis_id, id, port)
    }

    fn get_chassis_all<'a>(
        &'a self,
    ) -> crate::RedfishFuture<'a, Result<Vec<String>, RedfishError>> {
        self.g.get_chassis_all()
    }

    fn get_chassis<'a>(
        &'a self,
        id: &'a str,
    ) -> crate::RedfishFuture<'a, Result<crate::Chassis, RedfishError>> {
        self.g.get_chassis(id)
    }

    fn get_chassis_assembly<'a>(
        &'a self,
        chassis_id: &'a str,
    ) -> crate::RedfishFuture<'a, Result<crate::Assembly, RedfishError>> {
        self.g.get_chassis_assembly(chassis_id)
    }

    fn get_chassis_network_adapters<'a>(
        &'a self,
        chassis_id: &'a str,
    ) -> crate::RedfishFuture<'a, Result<Vec<String>, RedfishError>> {
        self.g.get_chassis_network_adapters(chassis_id)
    }

    fn get_chassis_network_adapter<'a>(
        &'a self,
        chassis_id: &'a str,
        id: &'a str,
    ) -> crate::RedfishFuture<'a, Result<crate::NetworkAdapter, RedfishError>> {
        self.g.get_chassis_network_adapter(chassis_id, id)
    }

    fn get_base_network_adapters<'a>(
        &'a self,
        system_id: &'a str,
    ) -> crate::RedfishFuture<'a, Result<Vec<String>, RedfishError>> {
        self.g.get_base_network_adapters(system_id)
    }

    fn get_base_network_adapter<'a>(
        &'a self,
        system_id: &'a str,
        id: &'a str,
    ) -> crate::RedfishFuture<'a, Result<crate::NetworkAdapter, RedfishError>> {
        self.g.get_base_network_adapter(system_id, id)
    }

    fn get_ports<'a>(
        &'a self,
        chassis_id: &'a str,
        network_adapter: &'a str,
    ) -> crate::RedfishFuture<'a, Result<Vec<String>, RedfishError>> {
        self.g.get_ports(chassis_id, network_adapter)
    }

    fn get_port<'a>(
        &'a self,
        chassis_id: &'a str,
        network_adapter: &'a str,
        id: &'a str,
    ) -> crate::RedfishFuture<'a, Result<crate::NetworkPort, RedfishError>> {
        self.g.get_port(chassis_id, network_adapter, id)
    }

    fn get_manager_ethernet_interfaces<'a>(
        &'a self,
    ) -> crate::RedfishFuture<'a, Result<Vec<String>, RedfishError>> {
        self.g.get_manager_ethernet_interfaces()
    }

    fn get_manager_ethernet_interface<'a>(
        &'a self,
        id: &'a str,
    ) -> crate::RedfishFuture<'a, Result<crate::EthernetInterface, RedfishError>> {
        self.g.get_manager_ethernet_interface(id)
    }

    fn get_system_ethernet_interfaces<'a>(
        &'a self,
    ) -> crate::RedfishFuture<'a, Result<Vec<String>, RedfishError>> {
        self.g.get_system_ethernet_interfaces()
    }

    fn get_system_ethernet_interface<'a>(
        &'a self,
        id: &'a str,
    ) -> crate::RedfishFuture<'a, Result<crate::EthernetInterface, RedfishError>> {
        self.g.get_system_ethernet_interface(id)
    }

    fn change_uefi_password<'a>(
        &'a self,
        current_uefi_password: &'a str,
        new_uefi_password: &'a str,
    ) -> crate::RedfishFuture<'a, Result<Option<String>, RedfishError>> {
        self.g
            .change_uefi_password(current_uefi_password, new_uefi_password)
    }

    fn get_job_state<'a>(
        &'a self,
        job_id: &'a str,
    ) -> crate::RedfishFuture<'a, Result<crate::JobState, RedfishError>> {
        self.g.get_job_state(job_id)
    }

    fn get_resource<'a>(
        &'a self,
        id: crate::model::ODataId,
    ) -> crate::RedfishFuture<'a, Result<crate::Resource, RedfishError>> {
        self.g.get_resource(id)
    }

    fn get_collection<'a>(
        &'a self,
        id: crate::model::ODataId,
    ) -> crate::RedfishFuture<'a, Result<crate::Collection, RedfishError>> {
        self.g.get_collection(id)
    }

    fn set_boot_order_dpu_first<'a>(
        &'a self,
        boot_interface: crate::BootInterfaceRef<'a>,
    ) -> crate::RedfishFuture<'a, Result<Option<String>, RedfishError>> {
        self.g.set_boot_order_dpu_first(boot_interface)
    }

    fn clear_uefi_password<'a>(
        &'a self,
        current_uefi_password: &'a str,
    ) -> crate::RedfishFuture<'a, Result<Option<String>, RedfishError>> {
        self.g.clear_uefi_password(current_uefi_password)
    }

    fn get_update_service<'a>(
        &'a self,
    ) -> crate::RedfishFuture<'a, Result<crate::model::update_service::UpdateService, RedfishError>>
    {
        self.g.get_update_service()
    }

    fn get_base_mac_address<'a>(
        &'a self,
    ) -> crate::RedfishFuture<'a, Result<Option<String>, RedfishError>> {
        self.g.get_base_mac_address()
    }

    fn lockdown_bmc<'a>(
        &'a self,
        target: crate::EnabledDisabled,
    ) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.lockdown_bmc(target)
    }

    fn is_ipmi_over_lan_enabled<'a>(
        &'a self,
    ) -> crate::RedfishFuture<'a, Result<bool, RedfishError>> {
        self.g.is_ipmi_over_lan_enabled()
    }

    fn enable_ipmi_over_lan<'a>(
        &'a self,
        target: crate::EnabledDisabled,
    ) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.enable_ipmi_over_lan(target)
    }

    fn enable_rshim_bmc<'a>(&'a self) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.enable_rshim_bmc()
    }

    fn clear_nvram<'a>(&'a self) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.clear_nvram()
    }

    fn get_nic_mode<'a>(
        &'a self,
    ) -> crate::RedfishFuture<
        'a,
        Result<Option<crate::model::oem::nvidia_dpu::NicMode>, RedfishError>,
    > {
        self.g.get_nic_mode()
    }

    fn set_nic_mode<'a>(
        &'a self,
        mode: crate::model::oem::nvidia_dpu::NicMode,
    ) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.set_nic_mode(mode)
    }

    fn enable_infinite_boot<'a>(&'a self) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        Box::pin(async move { self.s.enable_infinite_boot().await })
    }

    fn is_infinite_boot_enabled<'a>(
        &'a self,
    ) -> crate::RedfishFuture<'a, Result<Option<bool>, RedfishError>> {
        Box::pin(async move { self.s.is_infinite_boot_enabled().await })
    }

    fn set_host_rshim<'a>(
        &'a self,
        enabled: crate::EnabledDisabled,
    ) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.set_host_rshim(enabled)
    }

    fn get_host_rshim<'a>(
        &'a self,
    ) -> crate::RedfishFuture<'a, Result<Option<crate::EnabledDisabled>, RedfishError>> {
        self.g.get_host_rshim()
    }

    fn set_idrac_lockdown<'a>(
        &'a self,
        enabled: crate::EnabledDisabled,
    ) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.set_idrac_lockdown(enabled)
    }

    fn get_boss_controller<'a>(
        &'a self,
    ) -> crate::RedfishFuture<'a, Result<Option<String>, RedfishError>> {
        self.g.get_boss_controller()
    }

    fn decommission_storage_controller<'a>(
        &'a self,
        controller_id: &'a str,
    ) -> crate::RedfishFuture<'a, Result<Option<String>, RedfishError>> {
        self.g.decommission_storage_controller(controller_id)
    }

    fn create_storage_volume<'a>(
        &'a self,
        controller_id: &'a str,
        volume_name: &'a str,
    ) -> crate::RedfishFuture<'a, Result<Option<String>, RedfishError>> {
        self.g.create_storage_volume(controller_id, volume_name)
    }

    fn ac_powercycle_supported_by_power(&self) -> bool {
        self.g.ac_powercycle_supported_by_power()
    }

    fn is_boot_order_setup<'a>(
        &'a self,
        boot_interface: crate::BootInterfaceRef<'a>,
    ) -> crate::RedfishFuture<'a, Result<bool, RedfishError>> {
        self.g.is_boot_order_setup(boot_interface)
    }

    fn get_component_integrities<'a>(
        &'a self,
    ) -> crate::RedfishFuture<
        'a,
        Result<crate::model::component_integrity::ComponentIntegrities, RedfishError>,
    > {
        self.g.get_component_integrities()
    }

    fn get_firmware_for_component<'a>(
        &'a self,
        component_integrity_id: &'a str,
    ) -> crate::RedfishFuture<
        'a,
        Result<crate::model::software_inventory::SoftwareInventory, RedfishError>,
    > {
        self.g.get_firmware_for_component(component_integrity_id)
    }

    fn get_component_ca_certificate<'a>(
        &'a self,
        url: &'a str,
    ) -> crate::RedfishFuture<
        'a,
        Result<crate::model::component_integrity::CaCertificate, RedfishError>,
    > {
        self.g.get_component_ca_certificate(url)
    }

    fn trigger_evidence_collection<'a>(
        &'a self,
        url: &'a str,
        nonce: &'a str,
    ) -> crate::RedfishFuture<'a, Result<crate::model::task::Task, RedfishError>> {
        self.g.trigger_evidence_collection(url, nonce)
    }

    fn get_evidence<'a>(
        &'a self,
        url: &'a str,
    ) -> crate::RedfishFuture<'a, Result<crate::model::component_integrity::Evidence, RedfishError>>
    {
        self.g.get_evidence(url)
    }

    fn set_host_privilege_level<'a>(
        &'a self,
        level: crate::model::oem::nvidia_dpu::HostPrivilegeLevel,
    ) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.set_host_privilege_level(level)
    }

    fn set_utc_timezone<'a>(&'a self) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.set_utc_timezone()
    }

    fn set_ntp_servers<'a>(
        &'a self,
        servers: &'a [String],
    ) -> crate::RedfishFuture<'a, Result<(), RedfishError>> {
        self.g.set_ntp_servers(servers)
    }
}
