use std::collections::HashMap;

use commons::{db::Worker, GeoLocation};

pub fn assign_workers<'a>(
    workers: &'a [Worker],
    locations: impl Iterator<Item = &'a GeoLocation>,
) -> Result<Vec<&'a str>, ()> {
    #[inline]
    fn get_min_by<'b, 'c>(
        workers: &'b [Worker],
        worker_assign: &'c HashMap<&'b str, u32>,
        filter_by: impl Fn(&&Worker) -> bool,
    ) -> Option<&'b Worker> {
        workers
            .iter()
            .filter(filter_by)
            .min_by_key(|w| w.work_count + worker_assign.get(w._id.as_str()).cloned().unwrap_or(0))
    }

    let mut worker_assign: HashMap<&'a str, u32> = HashMap::new();
    let mut worker_ids = Vec::new();
    for location in locations {
        let worker = match location {
            GeoLocation::Name(name) => workers.iter().find(|w| w.name.eq(name)).ok_or(())?,
            GeoLocation::City(city) => {
                get_min_by(workers, &worker_assign, |w| w.city.eq(city)).ok_or(())?
            }
            GeoLocation::Country(country) => {
                get_min_by(workers, &worker_assign, |w| w.country.eq(country)).ok_or(())?
            }
            GeoLocation::Region(region) => {
                get_min_by(workers, &worker_assign, |w| w.region.eq(region)).ok_or(())?
            }
        };
        worker_ids.push(worker._id.as_str());
        let assigned_count = worker_assign.entry(worker._id.as_str()).or_insert(0);
        *assigned_count += 1;
    }

    Ok(worker_ids)
}
