#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod tipos;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use super::tipos::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	// Almacena los estados 
	#[pallet::storage]
	#[pallet::getter(fn etapa)]
	pub type EtapaDelJuego<T> = StorageValue<_, Etapa, ValueQuery>;

	// Almacena los jugadores
	#[pallet::storage]
	#[pallet::getter(fn jugadores)]
	pub type Jugadores<T> = StorageValue<_, BoundedVec<Jugador<T>, ConstU32<2>>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// El usuario se registró exitosamente.
		Registrado { quien: CuentaDe<T> },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// El usuario ya se registró para un juego, no puede volver a hacerlo.
		YaRegistrado,
		/// La etapa del juego es incorrecta
		EtapaIncorrecta,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Registra al usuario para jugar
		#[pallet::call_index(0)]
		#[pallet::weight(T::DbWeight::get().reads_writes(0, 10_000) + T::DbWeight::get().writes(1))]
		pub fn registrar(origen: OriginFor<T>) -> DispatchResult {
			// Revisar etapa del juego
			let mut etapa = EtapaDelJuego::<T>::get();
			ensure!(matches!(etapa, Etapa::EsperandoJugadores(_)), Error::<T>::EtapaIncorrecta);

			let quien = ensure_signed(origen)?;
			let mut jugadores = Jugadores::<T>::get();
			// Si la etapa es correcta, hay máximo un jugador en el arreglo.
			if let Some(primer_jugador) = jugadores.first() {
				ensure!(primer_jugador.0 != quien, Error::<T>::YaRegistrado);
			}

			let jugador = (quien.clone(), None, None); // Jugadores comienzan sin jugada ni compromiso.
			jugadores.force_push(jugador); // Sabemos que no está lleno el arreglo porque la etapa es correcta.
			Jugadores::<T>::set(jugadores);

			// Avanzar etapa
			etapa.next();
			EtapaDelJuego::<T>::set(etapa);

			Self::deposit_event(Event::Registrado { quien });
			Ok(())
		}
	}
}
