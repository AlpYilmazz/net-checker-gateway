use std::collections::HashMap;

use commons::{db::Worker, GeoLocation, GeoSettings};

pub fn assign_workers<'a>(
    workers: &'a [Worker],
    geo_options: &GeoSettings,
) -> Result<HashMap<&'a str, u32>, ()> {
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

    let locations = &geo_options.locations;

    let mut worker_assign: HashMap<&'a str, u32> = HashMap::new();
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
        let assigned_count = worker_assign.entry(worker._id.as_str()).or_insert(0);
        *assigned_count += 1;
    }

    Ok(worker_assign)
}
