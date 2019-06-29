import("../crate/pkg").then(module => {
  console.log(module);
  let cpu = module.return_cpu();
  console.log(cpu);
  cpu.cycle();
  module.greet("worlddd");
});
