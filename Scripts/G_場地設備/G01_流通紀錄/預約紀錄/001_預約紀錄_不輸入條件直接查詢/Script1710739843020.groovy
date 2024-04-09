import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://10.11.20.15/inspireapp/logon')

WebUI.maximizeWindow()

WebUI.setText(findTestObject('Object Repository/場地設備/流通紀錄/預約紀錄_不輸入條件查詢/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/場地設備/流通紀錄/預約紀錄_不輸入條件查詢/Page_(15trunk)/input_(15trunk)_member_pwd'), 
    'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/流通紀錄/預約紀錄_不輸入條件查詢/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/場地設備/流通紀錄/預約紀錄_不輸入條件查詢/Page_(15trunk)/a_'), '場地設備')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/流通紀錄/預約紀錄_不輸入條件查詢/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/場地設備/流通紀錄/預約紀錄_不輸入條件查詢/Page_(15trunk)/a__1'), '流通記錄')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/流通紀錄/預約紀錄_不輸入條件查詢/Page_(15trunk)/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/場地設備/流通紀錄/預約紀錄_不輸入條件查詢/Page_(15trunk)/a__1_2'), '預約記錄')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/流通紀錄/預約紀錄_不輸入條件查詢/Page_(15trunk)/a__1_2'))

WebUI.verifyElementText(findTestObject('Object Repository/場地設備/流通紀錄/預約紀錄_不輸入條件查詢/Page_(15trunk)/b_'), '查詢')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/流通紀錄/預約紀錄_不輸入條件查詢/Page_(15trunk)/img__ImageSearch'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/流通紀錄/預約紀錄_不輸入條件查詢/Page_(15trunk)/input__formSubmitSearch'))

WebUI.takeFullPageScreenshot()

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/流通紀錄/預約紀錄_不輸入條件查詢/Page_(15trunk)/a_2'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/流通紀錄/預約紀錄_不輸入條件查詢/Page_(15trunk)/a_3'))

WebUI.selectOptionByValue(findTestObject('Object Repository/場地設備/流通紀錄/預約紀錄_不輸入條件查詢/Page_(15trunk)/select_102050100'), '1', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/場地設備/流通紀錄/預約紀錄_不輸入條件查詢/Page_(15trunk)/select_102050100'), '2', 
    true)

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/流通紀錄/預約紀錄_不輸入條件查詢/Page_(15trunk)/li_GO'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/流通紀錄/預約紀錄_不輸入條件查詢/Page_(15trunk)/li_GO'))

WebUI.setText(findTestObject('Object Repository/場地設備/流通紀錄/預約紀錄_不輸入條件查詢/Page_(15trunk)/input__cp'), '3')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/流通紀錄/預約紀錄_不輸入條件查詢/Page_(15trunk)/span_GO'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/流通紀錄/預約紀錄_不輸入條件查詢/Page_(15trunk)/a_s_goToLastPage2'))

WebUI.selectOptionByValue(findTestObject('Object Repository/場地設備/流通紀錄/預約紀錄_不輸入條件查詢/Page_(15trunk)/select_102050100'), '0', 
    true)

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/流通紀錄/預約紀錄_不輸入條件查詢/Page_(15trunk)/a_s_goToLastPage2'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/流通紀錄/預約紀錄_不輸入條件查詢/Page_(15trunk)/a__1_2_3'))

WebUI.closeBrowser()

