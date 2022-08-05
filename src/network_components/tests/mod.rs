#[cfg(test)]
const MAC_IN_U8: &[u8] = &[224, 185, 229, 48, 239, 152];
//#[cfg(test)]
//const IPV4_IN_U8: &[u8] = &[192, 168, 1, 76];
#[cfg(test)]
const IPV4_DATA_IN_U8: [u8; 61] = [69, 0, 0, 61, 177, 29, 64, 0, 64, 17, 128, 107, 192, 168, 1, 90, 142, 250, 184, 42, 131, 149, 1, 187, 0, 41, 100, 18, 82, 18, 246, 2, 24, 57, 214, 254, 202, 113, 65, 255, 85, 173, 50, 221, 178, 53, 134, 231, 184, 197, 223, 157, 159, 28, 221, 181, 199, 230, 164, 142, 134];
#[cfg(test)]
const IPV4_DATA_IN_U8_WITH_OPTIONS: [u8; 61] = [74, 0, 0, 61, 177, 29, 64, 0, 64, 17, 128, 107, 192, 168, 1, 90, 142, 250, 184, 42, 131, 149, 1, 187, 0, 41, 100, 18, 82, 18, 246, 2, 24, 57, 214, 254, 202, 113, 65, 255, 85, 173, 50, 221, 178, 53, 134, 231, 184, 197, 223, 157, 159, 28, 221, 181, 199, 230, 164, 142, 134];
#[cfg(test)]
const UDP_DATA_IN_U8: [u8; 41] = [131, 149, 1, 187, 0, 41, 100, 18, 82, 18, 246, 2, 24, 57, 214, 254, 202, 113, 65, 255, 85, 173, 50, 221, 178, 53, 134, 231, 184, 197, 223, 157, 159, 28, 221, 181, 199, 230, 164, 142, 134];


#[cfg(test)]
pub mod mac_address_tests;

#[cfg(test)]
pub mod ethernet_packet_tests;


//#[cfg(test)]
//pub mod ipv4_address_tests;
#[cfg(test)]
pub mod ipv4_packet_tests;
#[cfg(test)]
pub mod ipv6_packet_tests;


#[cfg(test)]
pub mod tcp_packet_tests;

#[cfg(test)]
pub mod udp_packet_tests;


#[cfg(test)]
pub mod upper_layer_services_tests;
