use crate::hittable;
use crate::ray;

pub struct HittableList {
    objects: Vec<Box<dyn hittable::Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }

    pub fn add(&mut self, object: Box<dyn hittable::Hittable>) {
        self.objects.push(object);
    }
}

impl hittable::Hittable for HittableList {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::HitRecord> {
        struct ClosestObject {
            record: Option<hittable::HitRecord>,
            closest_so_far: f32,
        }

        let closest_object = self.objects.iter().fold(
            ClosestObject {
                record: None,
                closest_so_far: t_max,
            },
            |acc, object| {
                let hit = (*object).hit(&ray, t_min, acc.closest_so_far);
                match hit {
                    None => acc,
                    Some(record) => {
                        let t = record.t;

                        ClosestObject {
                            closest_so_far: t,
                            record: Some(record),
                        }
                    }
                }
            },
        );

        return closest_object.record;
    }
}
