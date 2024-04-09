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

WebUI.delay(2)

WebUI.setText(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.delay(2)

WebUI.setEncryptedText(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/input_(15trunk)_member_pwd'), 
    'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.click(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.click(findTestObject('New Page(15trunk)/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('New Page(15trunk)/Page_(15trunk)/a__1 - Copy'), '書單維護')

WebUI.click(findTestObject('New Page(15trunk)/Page_(15trunk)/a__1 - Copy'))

WebUI.maximizeWindow()

WebUI.verifyElementText(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/td_()'), '白雪壞公主(測試)')

WebUI.verifyElementPresent(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/div_9 records  Page 1 of 1 1               _ba7724'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/img__Any_1_8'), 0)

WebUI.click(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/img__Any_1_8'))

WebUI.takeFullPageScreenshotAsCheckpoint('刪除書單成功')

WebUI.click(findTestObject('New Page(15trunk)/Page_(15trunk)/a__1_2 - Copy'))

WebUI.closeBrowser()

