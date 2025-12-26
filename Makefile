#                                
#     ___  ___ ___ ___ 
#    / __|/ __/ _ \ __|
#    \__ \ (_|  __/__ \
#    |___/\___\___|___/
#                                
#  

PROFILE ?= debug

BASE ?= $(CURDIR)
ARCH ?= thumbv7em-none-eabihf

# Check the OS.
# If windows, use PowerShell command.
ifeq ($(OS),Windows_NT)
	ECHO     = echo
	RM       = powershell -Command "Remove-Item -Recurse -Force -ErrorAction SilentlyContinue"
	CONTINUE = || true
# By default, the Linux, use the shell.
else
	ECHO     = echo
	RM       = rm -rf
	CONTINUE = || true
endif

# Set the common used toolkit path.
CARGO = cargo
CMAKE = cmake

# RUST app build parameter.
APP      ?= main
APP_PATH ?= .

# RUST Cargo build profile parameter.
ifeq ($(PROFILE),release)
  RUST_BUILD_TYPE = --release
endif

# Set the platform settings.
PLATFORM      ?= 
PLATFORM_PATH ?= platform/${PLATFORM}

FEATURES ?=

# CMake platform build parameters.
CMAKE_BUILD      ?= build
CMAKE_BUILD_TYPE ?= Debug

ifeq ($(PROFILE),release)
  CMAKE_BUILD_TYPE = Release
endif

APP_CMAKE ?= $(PLATFORM_PATH)/app.cmake

all: platform_with_app

# Build the RUST based app code and generate a static library.
app:
	@$(ECHO) "-----------------------------   Compile App   -----------------------------"
	@$(CARGO) build --manifest-path $(APP_PATH)/Cargo.toml ${RUST_BUILD_TYPE} --features ${PLATFORM} ${FEATURES} --target ${ARCH}

# Clean the app generated target.
clean-app:
	@$(CARGO) clean --manifest-path $(APP_PATH)/Cargo.toml

# Build the C based platform code and generate the embedded image.
platform:
	@$(ECHO) "----------------------------- Compile Platform -----------------------------"
	@$(CMAKE) -S $(PLATFORM_PATH) -B $(CMAKE_BUILD) --preset $(CMAKE_BUILD_TYPE)
	@$(CMAKE) --build $(CMAKE_BUILD)

# Build the C based platform with RUST based app code and generate the embedded image.
platform_with_app: app
	@$(ECHO) ""

	@$(ECHO) "----------------------------- Compile Platform -----------------------------"
	@$(CMAKE) -S $(PLATFORM_PATH) -B $(CMAKE_BUILD) --preset $(CMAKE_BUILD_TYPE) \
		-DARCH=$(ARCH) -DPROFILE=$(PROFILE) -DBASE=$(BASE) \
		-DAPP=$(APP) -DAPP_PATH=$(APP_PATH) -DPLATFORM=$(PLATFORM) -C $(APP_CMAKE)
	@$(CMAKE) --build $(CMAKE_BUILD)

# Clena the platform generated target.
clean-platform:
	@$(MAKE) -C $(CMAKE_BUILD) clean $(CONTINUE)

# Clean the built target
clean: clean-platform clean-app

# Dist clean to direct remove target folder
distclean:
	@$(RM) $(CMAKE_BUILD) $(CONTINUE)
	@$(RM) $(RUST_BUILD) $(CONTINUE)

# Build image with the release profile.
release:
	@$(MAKE) PROFILE=release $(MAKEOVERRIDES)

.PHONY: all app clean release platform
