use std::net::Ipv4Addr;

#[derive(Debug)]
struct TrieNode {
    children: [Option<Box<TrieNode>>;2],
    is_end_node: bool,
    route: Option<String>,
}

impl TrieNode {


    pub fn new() -> TrieNode {

        TrieNode {
            children: [None, None],
            is_end_node: false,
            route: None,
        }
    }


    pub fn insert(&mut self, net_addr: Ipv4Addr, route: String, subnet_mask: usize) -> Result<String, String>{
        //get root node
        let mut node = self;

        //get the octet values
        let net_addr_oct = net_addr.octets();
        let mut count = 0usize;
        for eo in net_addr_oct {
            for i in (0..8).rev() {
                let bit = (eo >> i) & 1;
                //if there is no node just create
                //right for 1
                //left for 0
                if node.children[bit as usize].is_none() {
                    node.children[bit as usize] = Some(Box::new(TrieNode::new()));

                }
                if count == subnet_mask{
                    //mark as end node
                    node.is_end_node = true;
                    node.route = Some(route.clone());
                    return Ok(format!("Successfully added route {}",
                                      node.route.as_ref().unwrap()));
                }
                node = node.children[bit as usize].as_mut().unwrap();
                count += 1;
            }
        }

        Err(String::from("Could not add route"))
    }

    pub fn search(&self, dst_addr: &Ipv4Addr) -> Result<String, String> {
        let mut node = self;
        let dst_addr_oct = dst_addr.octets();

        //keep track of last route available
        let mut last_route: Option<String> = None;

        for oc in dst_addr_oct {
            for i in (0..8).rev() {
                let bit = (oc >> i) & 1;

                if let Some(next_node) = node.children[bit as usize].as_ref() {
                    node = next_node;
                    if node.is_end_node {
                        last_route = node.route.clone();
                    }
                } else {
                    break;
                }
            }
        }

        match last_route {
            Some(route) => Ok(route),
            None => Err(String::from("Complete No route available")),
        }
    }
}

fn main() {


    //create a trie node instance
    let trie_node = TrieNode::new();

    //insert some routes to trie node
    //192.168.0.0
    let ipv4_one = Ipv4Addr::new(192,168,0,0);
    let route_one = String::from("en01");
    let ipv4_two = Ipv4Addr::new(127,0,0,0);
    let route_two = String::from("en02");
    let ipv4_three = Ipv4Addr::new(192,168,0, 32);
    let route_three = String::from("en03");
    // insert to trie node
    //get mutable reference to trie node
    let mut trie_node_mut = trie_node;
    let result = trie_node_mut.insert(ipv4_one.clone(), route_one.clone(), 24);
    match result {
        Ok(success) => {
          println!("{}", success);
        },
        Err(_) => {},
    };

    let result = trie_node_mut.insert(ipv4_two.clone(), route_two.clone(), 24);
    match result {
        Ok(success) => {
            println!("{}", success);
        },
        Err(_) => {},
    };

    let result = trie_node_mut.insert(ipv4_three.clone(), route_three.clone(), 28);
    match result {
        Ok(success) => {
            println!("{}", success);
        },
        Err(_) => {},
    };

    //search 192.168.0.1
    let dst_ip = Ipv4Addr::new(192,168,0,33);
    let result = trie_node_mut.search(&dst_ip);
    match result {
        Ok(success) => {
            println!("{}", success);
        },
        Err(e) => {
            println!("{}", e);
        },
    };

    /*
    Output
    Successfully added route en01
    Successfully added route en02
    Successfully added route en03

    Result Route :

    en03
     */
}

