use timely::dataflow::*;
use timely::dataflow::operators::*;
use timely::dataflow::operators::probe::Handle as ProbeHandle;

use differential_dataflow::AsCollection;
use differential_dataflow::operators::*;
use differential_dataflow::lattice::Lattice;

use ::Collections;

// -- $ID$
// -- TPC-H/TPC-R Forecasting Revenue Change Query (Q6)
// -- Functional Query Definition
// -- Approved February 1998
// :x
// :o
// select
//     sum(l_extendedprice * l_discount) as revenue
// from
//     lineitem
// where
//     l_shipdate >= date ':1'
//     and l_shipdate < date ':1' + interval '1' year
//     and l_discount between :2 - 0.01 and :2 + 0.01
//     and l_quantity < :3;
// :n -1

pub fn query<G: Scope>(collections: &Collections<G>) -> ProbeHandle<G::Timestamp> 
where G::Timestamp: Lattice+Ord {

    collections
        .lineitems
        .filter(|x| x.ship_date >= ::types::create_date(1994, 1, 1) && 
                    x.ship_date < ::types::create_date(1995, 1, 1) && 
                    5 <= x.discount && 
                    x.discount < 7 && 
                    x.quantity < 24)
        .inner
        .map(|(x, time, diff)| ((), time, (x.extended_price * x.discount / 100) * diff as i64))
        .as_collection()
        .count()
        .probe()
        .0
}