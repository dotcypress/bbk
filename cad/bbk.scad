include <lib/9161c.scad>;

epsilon = 0.01;
base_height = 14;
base_width = 30;
base_length = 65;
bearing_r = 4.95;
m3_r = 1.47;
m2_r = 0.9;

module motors() {
  translate([base_width/2+5, base_length/2, 0])
    rotate([0, -90, 0]) 
    Motor9161c();

  translate([-base_width/2-5, -base_length/2, 0])
    rotate([0, 90, 0]) 
    Motor9161c();
}

module frame() {
  difference() {
    union() {
      translate([0, 0, 0])
        cube([base_width-epsilon, base_length, 3], center=true);

      hull() {
        translate([0, -base_length/2+9, 0])
          cube([base_width-epsilon, 1, 2], center=true);
        translate([0, -base_length/2, 0])
          rotate([0, 90, 0]) 
          cylinder(r=base_height/2, h=base_width-epsilon, center=true);
      }

      hull() {
        translate([0, base_length/2-9, 0])
          cube([base_width-epsilon, 1, 2], center=true);
        translate([0, base_length/2, 0])
          rotate([0, 90, 0]) 
          cylinder(r=base_height/2, h=base_width-epsilon, center=true);
      }

      translate([11.5, 29, 0]) 
        cylinder(r=2, h=base_height, center=true);
      translate([-11.5, 29, 0]) 
        cylinder(r=2, h=base_height, center=true);
      translate([11.5, -29, 0]) 
        cylinder(r=2, h=base_height, center=true);
      translate([-11.5, -29, 0]) 
        cylinder(r=2, h=base_height, center=true);
    }

    union() {
      motors();

      translate([base_width/2, -base_length/2, 0])
        rotate([0, 90, -1]) 
        cylinder(r=m3_r, h=base_width, center=true);
      translate([-base_width/2, base_length/2, 0])
        rotate([0, 90, -1]) 
        cylinder(r=m3_r, h=base_width, center=true);

      translate([3, -base_length/2+1.5, 0])
        rotate([0, 90, 0]) 
        cylinder(r=base_height/2-0.4, h=6, center=true);
      translate([-3, base_length/2-1.5, 0])
        rotate([0, 90, 0]) 
        cylinder(r=base_height/2-0.3, h=6, center=true);

      union() {
        translate([8, -16, 0]) 
          cylinder(r=m2_r, h=10, center=true);
        translate([-8, -16, 0]) 
          cylinder(r=m2_r, h=10, center=true);
        translate([8, 16, 0]) 
          cylinder(r=m2_r, h=10, center=true);
        translate([-8, 16, 0]) 
          cylinder(r=m2_r, h=10, center=true);

        translate([11.5, 29, 0]) 
          cylinder(r=m2_r, h=20, center=true);
        translate([-11.5, 29, 0]) 
          cylinder(r=m2_r, h=20, center=true);
        translate([11.5, -29, 0]) 
          cylinder(r=m2_r, h=20, center=true);
        translate([-11.5, -29, 0]) 
          cylinder(r=m2_r, h=20, center=true);
      }
    }
  }
}

module wheel(inner_r=bearing_r) {
  outer_r=12;
  width=12.8;
  intersection() {
    difference() {
      cylinder(r=outer_r, h=width, center=true);

      translate([0, 0, width/4])
        cylinder(r1=inner_r-0.1, r2=inner_r+0.1, h=width/2+epsilon, center=true);
      translate([0, 0, -width/4])
        cylinder(r1=inner_r+0.1, r2=inner_r-0.1, h=width/2+epsilon, center=true);
   
    }
    scale([1, 1, 1.7])
      sphere(r=outer_r);
  }
}

module motor_wheel() {
  translate([0, 0, 2]) 
    Motor9161cCoupler(height=4, rim_width=4);
  difference() {
    wheel();
    translate([0,0,4]) 
      cylinder(r=5.5, h=10, center=true);
  }
}

module demo() {
  $fn=32;

  %motors();
  frame();
  union() {
    translate([base_width/2+8, base_length/2, 0]) 
      rotate([0, 90, 0])
      motor_wheel();
    translate([base_width/2+8, -base_length/2, 0]) 
      rotate([0, -90, 0])
      wheel();

    translate([-base_width/2-8, base_length/2, 0]) 
      rotate([0, 90, 0])
      wheel();
    translate([-base_width/2-8, -base_length/2, 0]) 
      rotate([0, -90, 0])
      motor_wheel();
  }
}

module wheels_print() {
  $fn=128;
  translate([15, 15, 6.4])
    motor_wheel();
  translate([15, -15, 6.4])
    wheel();

  translate([-15, 15, 6.4])
    wheel();
  translate([-15, -15, 6.4])
    motor_wheel();
}

module frame_print() {
  $fn=128;
  translate([0, 0, base_width/2])
    rotate([0, 90, 0])
    frame();
}

*wheels_print();
*frame_print();
demo();