use std::time::Duration;

pub trait Program {
  type State;
  type Msg;
  type View;
  fn init(&self) -> Self::State;
  fn update(&self, state: Self::State, msg: Self::Msg) -> Self::State;
  fn view<T: FnMut(Self::Msg)>(&self, state: Self::State, dispatch: T) -> Self::View;
}

pub fn render_noop(view: ()) {
  println!("render noop");
}

pub fn noop_runtime<P: Program<State = (), Msg = (), View = ()>>(program: P) {
  // let update = program.update;
  // let view = program.view;
  // let done = program.done;
  let mut state = program.init();
  let mut update_queue = vec![];
  let dispatch =  |msg: ()| {
    let update = program.update(state, msg);
    update_queue.push(update);
  };

  let view = program.view(state, dispatch);
  render_noop(view);

  loop {
    std::thread::sleep(Duration::from_millis(1000));
    update_queue.iter().fold(state, |state, msg| program.update(state, *msg));
    let view = program.view(state, dispatch);
    render_noop(view);
  }


}