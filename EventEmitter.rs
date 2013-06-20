use core::hashmap::linear::LinearMap;

struct EventEmitter{
  priv events : LinearMap<~str, ~[@fn (@Event)]>
} 

struct Event {
  data : @Owned
}

impl Event {
  fn new(d : @Owned) -> @Event {
    @Event {
      data : d
    }
  }
}

impl EventEmitter {

  fn new() -> EventEmitter {
    EventEmitter {
      events : LinearMap::new()
    }
  }

  /*
    google says this is a compiker error...we shall see
   */
  fn on(&mut self, evt : ~str, handlr : @fn (@Event)){
    let evt_handlers = self.events.find_mut(&evt);
    match evt_handlers {
      Some(handlers) => {
        handlers.push(handlr);
      },
      None => {
        let mut handler_arr = ~[];
        handler_arr.push(handlr);
        self.events.insert(evt, handler_arr);
      }
    };
  }

  fn off(&mut self, evt : Option<~str>, optHandlr : Option<@fn (@Event)>){
    match evt {
      //No event name given 
      None => {
        match optHandlr {
          //emitter.off()
          None => self.events = LinearMap::new(),
          //emitter.off(myManagedFunc)
          Some(func) => {
            //this one will require more thought...lol
            return;
          }
        };
      },
      //Event name given
      Some(e) => {
        match optHandlr {
          //emitter.off("some::event")
          None => self.events.insert(e, ~[]),
          //emitter.off("some::event", myManagedFunc)
          Some(func) => {
            let mut events = self.events.find(&e);
            match events {
              //event is registered
              Some(handlers) => {
                let mut v = ~[];
                for handlers.each |&handler| {
                  if(true/*handler != func*/){ //needs to be fixed
                    v.push(handler);
                  }
                }
                self.events.insert(e, v)
              }, 
              //event isn't registered
              None => { return }
            } 
          }
        };
      }
    };
  }

  fn emit(&self, evt : ~str, data : @Owned){
    let evt_handlers = self.events.find(&evt);
    match evt_handlers {
      None => return,
      Some(handlers) => for handlers.each |&handle| {
        handle(Event::new(data));
      }
    };
  }

}

fn main() {
  
}

//*** both on and off suffer from the same error...google says it is a compiler issue
// error: loan of mutable field as mutable conflicts with prior loan