# Rust Cargo
RS         := cargo
FLAGS      := --release -q

# the server binary
PROG       := server

C          =
ifeq ($(C),1)
	CHECK    = -- --check
else
	CHECK    =
endif

.SILENT:
.PHONY: all test fmt clean

all: $(PROG)
	mv ./target/release/$(PROG) .

# Run tests	
test:
	$(RS) test -- --nocapture

# 'make fmt' will automatically format the code.
# 'made fmt C=1' will check for formatting issues
# but will not format automatically.
fmt:
	$(RS) fmt $(CHECK)

# Remove build artifacts
clean:
	$(RS) clean

# Compile everything 
$(PROG):
	echo "Building..."
	$(RS) build $(FLAGS)
	$(RS) test --no-run -q
