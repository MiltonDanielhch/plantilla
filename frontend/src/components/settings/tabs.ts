export function initTabs() {
  const tabBtns = document.querySelectorAll('.tab-btn')
  const tabContents = document.querySelectorAll('.tab-content')
  
  tabBtns.forEach(btn => {
    btn.addEventListener('click', () => {
      const tabId = btn.getAttribute('data-tab')
      
      tabBtns.forEach(b => {
        b.classList.remove('bg-primary', 'text-primary-foreground')
        b.classList.add('text-muted-foreground', 'hover:bg-accent', 'hover:text-accent-foreground')
      })
      btn.classList.remove('text-muted-foreground', 'hover:bg-accent', 'hover:text-accent-foreground')
      btn.classList.add('bg-primary', 'text-primary-foreground')
      
      tabContents.forEach(content => content.classList.add('hidden'))
      document.getElementById(`tab-${tabId}`)?.classList.remove('hidden')
    })
  })
}
