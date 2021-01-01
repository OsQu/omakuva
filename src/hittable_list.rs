use crate::hittable;
use crate::ray;

pub struct HittableList<'a> {
    objects: Vec<Box<dyn hittable::Hittable + 'a>>,
}

impl<'a> HittableList<'a> {
    pub fn new() -> HittableList<'a> {
        HittableList { objects: vec![] }
    }

    pub fn add(&mut self, object: Box<dyn hittable::Hittable + 'a>) {
        self.objects.push(object);
    }
}

impl<'a> hittable::Hittable for HittableList<'a> {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::HitRecord> {
        struct ClosestObject<'a> {
            record: Option<hittable::HitRecord<'a>>,
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
