use std::{cell::RefCell, rc::Rc};

type Callback<T> = Rc<RefCell<dyn FnMut(T)>>;

#[derive(Clone, Copy)]
pub struct DataParameter<T> {
    min: T,
    max: T,

    observed_min: T,
    observed_max: T,

    value: T,

    init_value: bool,
    // on_changed_listeners: Rc<RefCell<Vec<Callback<T>>>>,
}

impl<T> DataParameter<T>
where
    T: Copy + Clone + Default + Eq + Ord,
{
    pub fn new(min: T, max: T) -> Self {
        Self {
            min,
            max,
            init_value: false,
            observed_max: Default::default(),
            observed_min: Default::default(),
            value: Default::default(),
            // on_changed_listeners: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn set_value(&mut self, value: T) {
        if self.value != value {
            self.value = value;

            self.update_observed_values();
            self.changed();
        }
    }

    /// Checks if the observed minimum and maximum values have been exceeded, then updates them.
    ///
    /// If `self.init_value` is `false`, we first set the observed values to the current value.
    ///
    /// `self.init` value serves to stop the `Default::default()` value from taking precedence.
    fn update_observed_values(&mut self) {
        let value = self.value;

        if !self.init_value {
            self.observed_min = value;
            self.observed_max = value;

            self.init_value = true;
        } else {
            if value > self.observed_max {
                self.observed_max = value
            } else if value < self.observed_min {
                self.observed_min = value
            }
        }
    }

    pub fn value(&self) -> T {
        self.value
    }

    pub fn min(&self) -> T {
        self.min
    }

    pub fn max(&self) -> T {
        self.max
    }

    pub fn observed_min(&self) -> T {
        self.observed_min
    }

    pub fn observed_max(&self) -> T {
        self.observed_max
    }

    pub fn on_changed<F>(&self, callback: F)
    where
        F: FnMut(T) + 'static,
    {
        // self.on_changed_listeners
        //     .borrow_mut()
        //     .push(Rc::new(RefCell::new(callback)));
    }

    fn changed(&self) {
        // for callback in self.on_changed_listeners.borrow_mut().iter() {
        //     callback.borrow_mut()(self.value);
        // }
    }
}
