//! # Arbitor Crate
//! 
//! 'arbitor' is a collection of utilities for implementing arbitor blocks in cycle based 
//! simultors. The set of arbitors include
//! * Round Robin Arbitors
//!     * With and without priority reseting when idle
//! * Priority Arbitors (TBA)
//! * Multilevel Arbitors (TBA)
//! 
//! Example:
//! <pre><code>
//!     let mut rrarb = RRArb::new(10,false);
//!     rrarb.set_request(0);
//!     rrarb.set_request(3);
//!     rrarb.set_request(5);
//!     rrarb.text_display();
//!     let rrarb_iter = rrarb.clone().iter();
//!     for rq in rrarb_iter {
//!         println!("Requestor {}",rq);
//!         if rq == 0 {
//!             rrarb.grant(rq);
//!         }
//!     } 
//!     rrarb.text_display();
//!     let rrarb_iter = rrarb2.clone().iter();
//!     for rq in rrarb_iter {
//!         println!("Requestor {}",rq);
//!     }
//!     rrarb.text_display();
//! </code></pre>
//!  
//! Panics
//! * Granting a non-bidder 
//! 
//! Errors 
//! * arb() with no requestors
//! 
//! Safety 
//! * No comment
pub mod arbitor {
    #[derive(Debug)]
    #[derive(Clone)]
    pub struct RRArb {
        requestors : Vec<bool>, 
        prev_req_id : u64, 
        cfg_max_requestors: u64, 
        cfg_reset_on_idle : bool
    }   
    //pub struct RRArbIterator {
    //    index: u64;
    //}
    impl RRArb {
        // Public Functions
        /// Create a new Round Robin Arbitor
        pub fn new (max_requestors:u64,reset_on_idle:bool)-> RRArb {
            let mut init_requestors = Vec::new();
            // No Requestors after construction 
            for _i in 0..max_requestors {
                init_requestors.push(false);
            }
            RRArb {
                requestors: init_requestors, 
                prev_req_id: 0, // Set to position 0
                cfg_max_requestors: max_requestors,
                cfg_reset_on_idle: reset_on_idle,
            }
        }
        /// Indicate a bidder is contending for a shared resource.  Interator will include set requestors. 
        /// Arbitor will consider selecting this bidder after a request is set. Request must not already
        /// be set.
        pub fn set_request(&mut self, req_id:u64) {
            assert!(req_id < self.cfg_max_requestors);
            assert!(self.requestors[req_id as usize] == false);
            self.requestors[req_id as usize] = true;
        }
        /// Indicate a bidder is no longer contending for a shared resource.  Interator will not include cleared requestors. 
        /// Arbitor will notconsider selecting this bidder once cleared. Request must be set before clearing.
        pub fn clr_request(&mut self, req_id:u64) {
            assert!(req_id < self.cfg_max_requestors);
            assert!(self.requestors[req_id as usize] == true);
            self.requestors[req_id as usize] = false;
        }
        /// Select the next requestor based on the last requestor granted. If no granter, the return
        /// value will be None. Othersize Some(n) will hold the granted requestor.  
        /// The last granted indication will be updated.
        pub fn arb(&mut self) -> Option<u64> {
            let mut look = self.prev_req_id; 
            let mut look_cnt = 0;
            loop {
                // Advance Look
                if look == self.cfg_max_requestors-1 {
                    look = 0;
                }
                else {
                    look += 1;
                }
                self.prev_req_id = look;
                if self.requestors[look as usize] {
                    // Grant
                    self.requestors[look as usize] = false;
                    return Some(look)
                }
                if look_cnt == self.cfg_max_requestors {
                    return None 
                } 
                look_cnt += 1;
            }
        }
        /// Should be called each cycle, so resetting on idle functionality can up dated.
        pub fn step(&mut self){
            if self.cfg_reset_on_idle {
                if self.no_requestors() {
                    self.prev_req_id = 0;
                }
            }
        }
        /// Select the a specified requestor. The req_id must specify a active requestor.
        /// The last granted indication will be updated. Thre request will be cleared
        pub fn grant(&mut self,req_id:u64) {
            assert!(req_id < self.cfg_max_requestors);
            assert!(self.requestors[req_id as usize]);
            self.requestors[req_id as usize] = false;
            self.prev_req_id = req_id;
        } 
        /// Print the Arbitor State
        pub fn text_display (&self) {
            println!("{:?}",self)
        } 
        // Interators
        /// Produce an interator for walking through all the requestor.
        pub fn iter(self) -> RRArbIterator {
            RRArbIterator {
                rrarb: self,
                index: 0,
            }
        }
        // Private Functions
        fn no_requestors (&self)-> bool{
            for i in 0..self.cfg_max_requestors {
                if self.requestors[i as usize] {
                    return false
                }
            }
            true
        }
    }
    pub struct RRArbIterator {
        rrarb: RRArb,
        index: u64,
    }
    impl Iterator for RRArbIterator {
        type Item = u64;
        fn next(&mut self) -> Option<Self::Item> {
            let mut position: u64;
            if self.index == self.rrarb.cfg_max_requestors {
                return None;
            }
            else {
                loop {
                    if self.index == self.rrarb.cfg_max_requestors {
                        return None;
                    }
                    else {
                        self.index += 1;
                        position = (self.rrarb.prev_req_id + self.index) % self.rrarb.cfg_max_requestors;
                        if self.rrarb.requestors[position as usize] { 
                            return Some(position);
                        }
                    }
                }
            }
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn select_middle() {
            let mut rrarb = RRArb::new(10,false);
            rrarb.set_request(5);
            rrarb.set_request(4);
            rrarb.clr_request(4);
            let selected = rrarb.arb();
            //match selected {
            //    Some(5) => println!("Yes. It works!"),
            //    Some(n) => {println!("Selected Incorrectly {}",n); panic!("Bad News")},
            //    None => println!("No Selection") 
            //}
            assert!(selected == Some(5))
        }
        #[test]
        fn select_none() {
            let mut rrarb = RRArb::new(100,false);
            rrarb.set_request(0);
            rrarb.set_request(10);
            let mut selected = rrarb.arb();
            assert!(selected == Some(10));
            selected = rrarb.arb();
            assert!(selected == Some(0));
            selected = rrarb.arb();
            assert!(selected == None);
        }
        #[test]
        fn test_iter() {
            let mut rrarb = RRArb::new(100,false);
            rrarb.set_request(0);
            rrarb.set_request(10);
            rrarb.text_display();
            let mut grant_order: u64 = 0;
            let rrarb_iter = rrarb.clone().iter();
            for rq in rrarb_iter {
                println!("Requestor {}",rq);
                match grant_order {
                    0=> assert!(rq == 10),
                    1=> assert!(rq == 0),
                    _=> assert!(false)
                }
                grant_order +=1;
            } 
        }
    }
}