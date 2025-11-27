#include <algorithm>
#include <numeric>
#include <iostream>
#include <string>
#include <vector>

#include "catch.hpp"

using namespace std;

#include <memory>

template <typename T>
class LifoList {
private:
    struct Node {
        T data;
        std::unique_ptr<Node> next;

        Node(const T& value) : data(value), next(nullptr) {}
    };

    std::unique_ptr<Node> head_;
    size_t size_;
public:
    LifoList() : head_(nullptr), size_(0) {}

    void push(const T& value) {
        auto newNode = std::make_unique<Node>(value);
        newNode->next = std::move(head_);
        head_ = std::move(newNode);
        ++size_;
    }

    void pop() {
        if (head_) {
            head_ = std::move(head_->next);
            --size_;
        }
    }

    T& top() {
        if (head_) {
            return head_->data;
        }
        throw std::runtime_error("LifoList is empty");
    }

    size_t size() const {
        return size_;
    }

    bool empty() const {
        return size_ == 0;
    }
};

TEST_CASE("LifoList basic operations", "[LifoList]") {
    LifoList<int> list;

    REQUIRE(list.empty());
    REQUIRE(list.size() == 0);

    list.push(10);
    REQUIRE(!list.empty());
    REQUIRE(list.size() == 1);
    REQUIRE(list.top() == 10);

    list.push(20);
    REQUIRE(list.size() == 2);
    REQUIRE(list.top() == 20);

    list.pop();
    REQUIRE(list.size() == 1);
    REQUIRE(list.top() == 10);

    list.pop();
    REQUIRE(list.empty());
    REQUIRE(list.size() == 0);
}

TEST_CASE("LifoList with strings", "[LifoList]") {
    LifoList<std::string> list;

    list.push("first");
    list.push("second");
    list.push("third");

    REQUIRE(list.size() == 3);
    REQUIRE(list.top() == "third");

    list.pop();
    REQUIRE(list.top() == "second");

    list.pop();
    REQUIRE(list.top() == "first");

    list.pop();
    REQUIRE(list.empty());
}

TEST_CASE("LifoList exception on top from empty list", "[LifoList]") {
    LifoList<int> list;

    REQUIRE_THROWS_AS(list.top(), std::runtime_error);
}

TEST_CASE("Stress test LifoList", "[LifoList]") {
    LifoList<int> list;
    const int N = 1'000'000;

    for (int i = 0; i < N; ++i) {
        list.push(i);
    }
}