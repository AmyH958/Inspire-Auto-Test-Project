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

WebUI.setText(findTestObject('Object Repository/流通/指定參考書/不輸入直接存檔/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/流通/指定參考書/不輸入直接存檔/Page_(15trunk)/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/不輸入直接存檔/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/指定參考書/不輸入直接存檔/Page_(15trunk)/a_'), '流通')

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/不輸入直接存檔/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/指定參考書/不輸入直接存檔/Page_(15trunk)/a__1'), '指定參考書')

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/不輸入直接存檔/Page_(15trunk)/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/指定參考書/不輸入直接存檔/Page_(15trunk)/a__1_2'), '新增')

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/不輸入直接存檔/Page_(15trunk)/a__1_2'))

WebUI.switchToWindowUrl('http://10.11.20.15/inspireapp/AcademicReservedBook,searchComponent.$DirectLink.sdirect')

WebUI.switchToWindowTitle('iNspire')

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/不輸入直接存檔/Page_iNspire/a_'))

WebUI.closeWindowUrl('http://10.11.20.15/inspireapp/AcademicReservedBook,searchComponent.$DirectLink.sdirect')

WebUI.switchToWindowTitle('神資圖書館(15trunk機)')

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/不輸入直接存檔/Page_(15trunk)/a__1_2_3'))

WebUI.closeBrowser()

