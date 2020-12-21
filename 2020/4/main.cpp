#include <iostream>
#include <fstream>
#include <optional>
#include <set>
#include <sstream>
#include <string>

template <typename T>
using Option = std::optional<T>;

namespace std {

std::string to_string(std::string& s) {
    return s;
}

template <typename T>
std::string to_string(Option<T> o) {
    return o.has_value() ? std::to_string(o.value()) : "NULL";
}

template <typename T>
std::string to_string(Option<T>& o) {
    return to_string(o);
}

} // namespace std


class Passport {
public:
    Passport() {}

    bool addInfo(std::string& kv) {
        std::istringstream iss{kv};

        std::string key;
        std::string value;
        if (!std::getline(iss, key, ':'))
            return false;
        if (!std::getline(iss, value, ':'))
            return false;

        // make sure there is no other text past key:value
        std::string next;
        if (iss >> next) {
            std::cerr << "Error: found unexpected text after KEY:VALUE pair: " 
                << next << std::endl;
            return false;
        }

        // based on key, parse value properly and store in passport structure
        if (key == "byr") {
            try {
                byr_ = stoul(value);
            } catch (std::exception e) {
            }

            if ( !(1920 <= *byr_ && *byr_ <= 2002) ) {
                byr_.reset();
            }
        } else if (key == "iyr") {
            try {
                iyr_ = stoul(value);
            } catch (std::exception e) {
            }

            if ( !(2010 <= *iyr_ && *iyr_ <= 2020) ) {
                iyr_.reset();
            }
        } else if (key == "eyr") {
            try {
                eyr_ = stoul(value);
            } catch (std::exception e) {
            }

            if ( !(2020 <= *eyr_ && *eyr_ <= 2030) ) {
                eyr_.reset();
            }
        } else if (key == "hgt") {
            try {
                size_t units = 0;
                uint_least16_t magnitude = stoul(value, &units);
                
                if (value.substr(units) == "cm") {
                    if (150 <= magnitude && magnitude <= 193) {
                        hgt_ = value;
                    }
                } else if (value.substr(units) == "in") {
                    if (59 <= magnitude && magnitude <= 76) {
                        hgt_ = value;
                    }
                }
            } catch (std::exception e) {
            }
        } else if (key == "hcl") {
            hcl_ = value;

            if (hcl_->length() != 7 || (*hcl_)[0] != '#') {
                hcl_.reset();
            } else {
                for (char c : hcl_->substr(1)) {
                    if (! (('0' <= c && c <= '9') || ('a' <= c && c <= 'f'))) {
                        hcl_.reset();
                        break;
                    }
                }
            }
        } else if (key == "ecl") {
            ecl_ = value;

            std::set<std::string> allowed = {
                "amb", "blu", "brn", "gry", "grn", "hzl", "oth" };
            if (allowed.find(*ecl_) == allowed.end()) {
                ecl_.reset();
            }
        } else if (key == "pid") {
            bool valid = value.length() == 9;
            if (valid) {
                for (char c : value) {
                    if (!isdigit(c)) {
                        valid = false;
                        break;
                    }
                }
            }
            if (valid) {
                try {
                    pid_ = stoull(value);
                } catch (std::exception e) {
                }
            }
        } else if (key == "cid") {
            try {
                cid_ = stoull(value);
            } catch (std::exception e) {
            }
        } else {
            std::cerr << "Error: found invalid key: " << key << std::endl;
            return false;
        }

        return true;
    }

    operator std::string() const {
        std::string ret;

        ret +=  "=== Passport: ===\n"; 
        ret += "byr: " + std::to_string(byr_) + "\n";
        ret += "iyr: " + std::to_string(iyr_) + "\n";
        ret += "eyr: " + std::to_string(eyr_) + "\n";
        ret += "hgt: " + std::to_string(hgt_) + "\n";
        ret += "hcl: " + std::to_string(hcl_) + "\n";
        ret += "ecl: " + std::to_string(ecl_) + "\n";
        ret += "pid: " + std::to_string(pid_) + "\n";
        ret += "cid: " + std::to_string(cid_) + "\n";

        return ret;
    }

    bool valid() const {
        return byr_.has_value() &&
               iyr_.has_value() &&
               eyr_.has_value() &&
               hgt_.has_value() &&
               hcl_.has_value() &&
               ecl_.has_value() &&
               pid_.has_value();
               // cid intentionally ignored
    }

private:
    Option<uint_fast16_t> byr_;
    Option<uint_fast16_t> iyr_;
    Option<uint_fast16_t> eyr_;
    Option<std::string>   hgt_;
    Option<std::string>   hcl_;
    Option<std::string>   ecl_;
    Option<uint64_t>      pid_;
    Option<uint16_t>      cid_;
};

std::ostream& operator<<(std::ostream& out, const Passport& p) {
    return out << std::string(p);
}

int main(void) {
    std::ifstream file{"input.txt"};    

    if (!file) {
        std::cerr << "Error, could not open input file" << std::endl;
        return 1;
    }

    Passport p;
    size_t valid = 0;

    while (file) {
        std::string line;
        std::getline(file, line);

        if (line.empty()) {
            valid += p.valid() ? 1 : 0;
            p = {};
        }

        std::istringstream iss{line};
        std::string token{};

        while (iss >> token) {
            p.addInfo(token);
        }
    }

    std::cout << valid << std::endl;
    
    return 0;
}
