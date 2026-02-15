import { atom, computed } from 'nanostores'
import type { PaginatedResponse } from '../types'

// Tipos para la tabla
export interface TableState<T> {
  data: T[]
  loading: boolean
  error: string | null
  pagination: {
    page: number
    limit: number
    total: number
    totalPages: number
  }
  sorting: {
    column: string | null
    direction: 'asc' | 'desc'
  }
  filters: Record<string, string>
  selectedRows: Set<number>
}

// Estado inicial
export function createTableState<T>(): TableState<T> {
  return {
    data: [],
    loading: false,
    error: null,
    pagination: {
      page: 1,
      limit: 10,
      total: 0,
      totalPages: 0,
    },
    sorting: {
      column: null,
      direction: 'asc',
    },
    filters: {},
    selectedRows: new Set(),
  }
}

// Store factory para crear stores de tabla tipados
export function createTableStore<T extends { id: number }>() {
  const $state = atom<TableState<T>>(createTableState<T>())

  // Computed values
  const $hasSelection = computed($state, (state) => state.selectedRows.size > 0)
  const $selectedCount = computed($state, (state) => state.selectedRows.size)
  const $isAllSelected = computed($state, (state) => 
    state.data.length > 0 && state.selectedRows.size === state.data.length
  )

  // Actions
  function setLoading(loading: boolean) {
    $state.set({ ...$state.get(), loading })
  }

  function setError(error: string | null) {
    $state.set({ ...$state.get(), error })
  }

  function setData(response: PaginatedResponse<T>) {
    $state.set({
      ...$state.get(),
      data: response.data,
      pagination: {
        page: response.page,
        limit: response.limit,
        total: response.total,
        totalPages: response.total_pages,
      },
      loading: false,
      error: null,
    })
  }

  function setPage(page: number) {
    const state = $state.get()
    $state.set({
      ...state,
      pagination: { ...state.pagination, page },
    })
  }

  function setLimit(limit: number) {
    const state = $state.get()
    $state.set({
      ...state,
      pagination: { ...state.pagination, limit, page: 1 },
    })
  }

  function setSorting(column: string) {
    const state = $state.get()
    const direction = state.sorting.column === column && state.sorting.direction === 'asc' 
      ? 'desc' 
      : 'asc'
    
    $state.set({
      ...state,
      sorting: { column, direction },
    })
  }

  function setFilter(column: string, value: string) {
    const state = $state.get()
    const filters = { ...state.filters }
    
    if (value) {
      filters[column] = value
    } else {
      delete filters[column]
    }
    
    $state.set({
      ...state,
      filters,
      pagination: { ...state.pagination, page: 1 },
    })
  }

  function clearFilters() {
    const state = $state.get()
    $state.set({
      ...state,
      filters: {},
      pagination: { ...state.pagination, page: 1 },
    })
  }

  function toggleSelection(id: number) {
    const state = $state.get()
    const selectedRows = new Set(state.selectedRows)
    
    if (selectedRows.has(id)) {
      selectedRows.delete(id)
    } else {
      selectedRows.add(id)
    }
    
    $state.set({ ...state, selectedRows })
  }

  function toggleAllSelection() {
    const state = $state.get()
    const allIds = state.data.map((row) => row.id)
    const allSelected = allIds.every((id) => state.selectedRows.has(id))
    
    const selectedRows = allSelected 
      ? new Set<number>() 
      : new Set(allIds)
    
    $state.set({ ...state, selectedRows })
  }

  function clearSelection() {
    $state.set({ ...$state.get(), selectedRows: new Set() })
  }

  function reset() {
    $state.set(createTableState<T>())
  }

  return {
    $state,
    $hasSelection,
    $selectedCount,
    $isAllSelected,
    setLoading,
    setError,
    setData,
    setPage,
    setLimit,
    setSorting,
    setFilter,
    clearFilters,
    toggleSelection,
    toggleAllSelection,
    clearSelection,
    reset,
  }
}
