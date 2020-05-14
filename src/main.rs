use async_trait::async_trait;
use cucumber::{after, before, CucumberBuilder, Steps, StepsBuilder, TestFuture};

#[tokio::main]
async fn main() {
    let mut builder = CucumberBuilder::new(cucumber::DefaultOutput::default());

    builder
        .features(vec!["./features".into()])
        .setup(setup)
        .steps(steps());

    builder.command_line().await;
}

// Declares a before handler function named `a_before_fn`
before!(a_before_fn => |_scenario| {

});

// Declares an after handler function named `an_after_fn`
after!(an_after_fn => |_scenario| {

});

// A setup function to be called before everything else
fn setup() {}

pub struct MyWorld {
    foo: String,
}

impl MyWorld {
    async fn test_async_fn(&mut self) /*-> Result<()>*/
    {
        println!("This is MyWorld::test_async_fn()");
        panic!("this is done");
        // Ok(())
    }
}

#[async_trait]
impl cucumber::World for MyWorld {
    async fn new() -> Self {
        MyWorld { foo: "foo".into() }
    }
}

pub fn steps() -> Steps<MyWorld> {
    let mut builder: StepsBuilder<MyWorld> = StepsBuilder::new();

    builder
        .given_async("Given Number 1", |world, _step| {
            let world = world.clone();
            TestFuture::new(async move {
                let mut world = world.write().unwrap();
                world.foo = "bla".into()
            })
        })
        // *****************************************
        .given_async("Example from GitHub comment", |world, _step| {
            let world = world.to_owned();
            TestFuture::new(async move {
                let world = world; // world is owned now
            })
        })
        .given_async("Modified example from GitHub comment", |world, _step| {
            let world = world.to_owned();
            TestFuture::new(async move {
                let world = world; // world is owned now
                let mut world = world.write().unwrap();
                assert_eq!(world.foo, "bla");
                // Uncommenting this will prevent compilation with: `&mut MyWorld` may not be safely transferred across an unwind boundary
                // world.test_async_fn().await
            })
        })
        // *****************************************
        .given_async("This one compiles fines", |world, _step| {
            TestFuture::new(async move {
                let mut world = world.write().unwrap();
                assert_eq!(world.foo, "bla")
            })
        })
        // // Fails to compile with: `&mut MyWorld` may not be safely transferred across an unwind boundary
        // .given_async("This one fails to compile", |world, _step| {
        //     TestFuture::new(async move {
        //         let mut world = world.write().unwrap();
        //         assert_eq!(world.foo, "bla");
        //         world.test_async_fn().await
        //     })
        // })
        .when_async("When Number 1", |world, _step| {
            TestFuture::new(async move {
                let mut world = world.write().unwrap();
                assert_eq!(world.foo, "bla")
            })
        })
        .then_async("Then Number 1", |world, _step| {
            TestFuture::new(async move {
                let mut world = world.write().unwrap();
            })
        });

    builder.build()
}
