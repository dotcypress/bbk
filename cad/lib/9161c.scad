use <gears.scad>

module Motor9161cCoupler(height = 5, rim_width = 1) {
  rotate([0, 0, 16]) translate([0, 0, -height - 0.01]) 
    ring_gear(0.444, 11, height, rim_width);
}

module Motor9161c(contacts = false) {
  rotate([0, 90, 0]) translate([3.6, 0, 0]) {
    translate([-3.6, 0, 0]) rotate([0, 90, 0]) union() {
      cylinder(r=0.6, h=5.6);
      spur_gear(0.314, 11, 3.6, 0);
    }
    translate([-3.6, 0, 0]) rotate([0, -90, 0]) union() {
      translate([0, 0, 21]) cylinder(r=0.5, h=4);
      cylinder(r=5.4, h=7);
      translate([0, 0, 6.99]) intersection() {
        cylinder(r=5.2, h=15);
        translate([-4.1, -6, 0]) cube([8.2, 12.01, 14]);
      }
      translate([0, 0, 21]) cylinder(r=2, h=1);
      if (contacts) {
        union() {
          translate([-4, -5, 7]) cube([2, 2, 14]);
          translate([2, -5, 7]) cube([2, 2, 14]);
        }
      }
    }
  }
}

