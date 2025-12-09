#include <algorithm>
#include <fstream>
#include <iostream>
#include <vector>

using std::vector;

class Box {
  double x, y, z;

public:
  void set_coords(int x_, int y_, int z_) {
    x = x_;
    y = y_;
    z = z_;
  }

  double distance(const Box &other) const {
    auto dx = x - other.x, dy = y - other.y, dz = z - other.z;
    return dx * dx + dy * dy + dz * dz;
  }

  double get_x() const { return x; }
};

static std::istream &operator>>(std::istream &is, Box &box) {
  int x, y, z;
  char c;
  is >> x >> c >> y >> c >> z;
  box.set_coords(x, y, z);
  return is;
}

class DistanceSpec {
  int fst, snd;
  double dist;

public:
  DistanceSpec(int fst_, int snd_, double dist_)
      : fst(fst_), snd(snd_), dist(dist_) {}

  int first() const { return fst; }
  int second() const { return snd; }
  bool operator<(const DistanceSpec &other) const { return dist < other.dist; }
};

class CircuitMap {
  vector<Box> all_boxes;
  vector<int> parents;

public:
  CircuitMap(vector<Box> &&boxes) : all_boxes(std::move(boxes)) {
    for (int i = 0; i < all_boxes.size(); i++) {
      parents.push_back(i);
    }
  }

  const vector<Box> &boxes() const { return all_boxes; }

  int get_root(int x) {
    return parents[x] == x ? x : parents[x] = get_root(parents[x]);
  }

  bool connect(int fst, int snd) {
    fst = get_root(fst);
    snd = get_root(snd);
    parents[fst] = snd;
    return fst != snd;
  }

  vector<int> sizes() {
    vector<int> out(all_boxes.size());
    for (int i = 0; i < all_boxes.size(); i++)
      out[get_root(i)]++;
    return out;
  }
};

int main() {
  vector<Box> boxes;
  Box b;
  std::fstream in("./input8.txt");
  while (in >> b) {
    boxes.push_back(b);
  }
  vector<DistanceSpec> distances;
  for (int i = 0; i < boxes.size(); i++) {
    for (int j = 0; j < i; j++) {
      distances.push_back(DistanceSpec(i, j, boxes[i].distance(boxes[j])));
    }
  }
  std::sort(distances.begin(), distances.end());

  CircuitMap cm(std::move(boxes));
  for (int i = 0; i < 1000; i++) {
    cm.connect(distances[i].first(), distances[i].second());
  }
  auto sizes = cm.sizes();

  int connections_remaining = -1;
  for (auto it : sizes) {
    if (it > 0)
      connections_remaining++;
  }
  std::sort(sizes.begin(), sizes.end());
  int day1 = 1;
  for (int k = 0; k < 3; k++) {
    day1 *= sizes.back();
    sizes.pop_back();
  }

  long day2 = 1;
  for (int i = 1000; i < distances.size(); i++) {
    if (cm.connect(distances[i].first(), distances[i].second())) {
      connections_remaining--;
    }
    if (!connections_remaining) {
      day2 = cm.boxes()[distances[i].first()].get_x() *
             cm.boxes()[distances[i].second()].get_x();
      break;
    }
  }

  std::cout << "Day 1: " << day1 << "\n";
  std::cout << "Day 2: " << day2 << "\n";
}
