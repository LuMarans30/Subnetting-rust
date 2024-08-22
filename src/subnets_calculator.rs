use crate::subnet::{Subnet, SubnetError};

pub struct SubnetCalculator {
    pub subnets: Vec<Subnet>,
    num_hosts_array: Vec<u32>,
}

impl SubnetCalculator {
    pub fn new(num_hosts_array: Vec<u32>) -> SubnetCalculator {
        SubnetCalculator {
            subnets: Vec::new(),
            num_hosts_array,
        }
    }

    /// Calculates the subnet for each number of hosts in the array
    pub fn calculate(&mut self, network: &str, cidr: u32) -> Result<(), SubnetError> {
        self.num_hosts_array.sort_by(|a, b| b.cmp(a));

        let mut network_tmp = network.to_string();
        let mut cidr_tmp = cidr;

        for num_hosts in self.num_hosts_array.iter() {
            let mut subnet = Subnet::new(&network_tmp, cidr_tmp, *num_hosts)?;
            subnet.calculate()?;
            self.subnets.push(subnet);

            let next_network_tmp = subnet.next_subnet.to_string();
            network_tmp = next_network_tmp;
            cidr_tmp = subnet.next_cidr;
        }

        Ok(())
    }
}
